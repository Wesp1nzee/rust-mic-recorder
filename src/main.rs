use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use hound::{WavSpec, WavWriter, SampleFormat};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Не найдено устройства ввода");
    println!("Используется устройство: {}", device.name()?);

    // конфиг потока
    let config: StreamConfig = device.default_input_config()?.into();
    if device.default_input_config()?.sample_format() != cpal::SampleFormat::F32 {
        eprintln!("Требуется устройство с форматом F32");
    }

    // Спецификация WAV
    let spec = WavSpec {
        channels: config.channels,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    // WAV писатель
    let writer = WavWriter::create("recording.wav", spec)?;
    let writer = Arc::new(Mutex::new(writer));

    // запись в WAV
    let writer_cb = writer.clone();
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut guard = writer_cb.lock().unwrap();
            for &sample in data {
                if let Err(e) = guard.write_sample(sample) {
                    eprintln!("Ошибка записи в WAV: {}", e);
                }
            }
        },
        move |err| eprintln!("Ошибка потока: {:?}", err),
        None,
    )?;

    stream.play()?;
    println!("Захват начат. Запись в recording.wav… (Ctrl+C для выхода)");

    let running = Arc::new(Mutex::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let mut run = r.lock().unwrap();
        *run = false;
    }).expect("Ошибка установки обработчика Ctrl+C");    
    Ok(while *running.lock().unwrap() {
        std::thread::sleep(Duration::from_secs(1));
    })
}
