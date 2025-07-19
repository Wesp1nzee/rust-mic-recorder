use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use hound::{WavSpec, WavWriter, SampleFormat as HoundFormat};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // хост и устройство
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Не найдено устройства ввода");
    println!("Используется устройство: {}", device.name()?);

    // получаем конфиг устройсвт 
    let supported_config = device.default_input_config()?;
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.clone().into();

    // спецификация WAV (32 бит флоат)
    let spec = WavSpec {
        channels: config.channels,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 32,
        sample_format: HoundFormat::Float,
    };
    let writer = WavWriter::create("recording.wav", spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    let stream = match sample_format {
        SampleFormat::F32 => {
            let w = writer.clone();
            device.build_input_stream(
                &config,
                move |data: &[f32], _| {
                    let mut guard = w.lock().unwrap();
                    for &s in data {
                        if let Err(e) = guard.as_mut().unwrap().write_sample(s) {
                            eprintln!("Ошибка записи: {}", e);
                        }
                    }
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::I16 => {
            let w = writer.clone();
            device.build_input_stream(
                &config,
                move |data: &[i16], _| {
                    let mut guard = w.lock().unwrap();
                    for &s in data {
                        let f = s as f32 / i16::MAX as f32;
                        if let Err(e) = guard.as_mut().unwrap().write_sample(f) {
                            eprintln!("Ошибка записи: {}", e);
                        }
                    }
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::U16 => {
            let w = writer.clone();
            device.build_input_stream(
                &config,
                move |data: &[u16], _| {
                    let mut guard = w.lock().unwrap();
                    for &s in data {
                        // u16 ползунок от 0..u16::MAX → -1.0..+1.0
                        let f = (s as f32 / u16::MAX as f32) * 2.0 - 1.0;
                        if let Err(e) = guard.as_mut().unwrap().write_sample(f) {
                            eprintln!("Ошибка записи: {}", e);
                        }
                    }
                },
                err_fn,
                None,
            )?
        },
        _ => {
            return Err("Неподдерживаемый формат аудио".into());
        }
    };

    // Запускаем поток
    stream.play()?;
    println!("Захват начат. (Ctrl+C для остановки)");

    let running = Arc::new(Mutex::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            let mut flag = r.lock().unwrap();
            *flag = false;
        })?;
    }

    while *running.lock().unwrap() {
        std::thread::sleep(Duration::from_secs(1));
    }
    drop(stream); 
    let mut w = writer.lock().unwrap();
    let writer = w.take().unwrap();
    writer.finalize()?;

    println!("Запись завершена в файл recording.wav");
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("Ошибка потока: {}", err);
}
