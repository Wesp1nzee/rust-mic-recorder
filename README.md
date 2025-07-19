# rust-mic-recorder 🇺🇸

Simple command-line utility written in Rust for recording audio from your microphone to a WAV file (`recording.wav`).

## Features

- Automatically selects the default input device.
- Records as WAV with 32-bit floating point samples (F32).
- Gracefully handles termination with Ctrl+C.

## Usage

1. **Build the project:**
   ```sh
   cargo build --release
   ```

2. **Run the program:**
   ```sh
   cargo run --release
   ```
   Recording starts automatically, and the output file is `recording.wav` in the current directory.

3. **Stop recording** by pressing `Ctrl+C`.

## Dependencies

- [`cpal`](https://crates.io/crates/cpal) — access to audio devices.
- [`hound`](https://crates.io/crates/hound) — writing WAV files.
- [`ctrlc`](https://crates.io/crates/ctrlc) — handling termination signal.

## Notes

- Requires an audio device supporting F32 format.
- By default, the system’s default input device is selected.

## License

MIT


---

# rust-mic-recorder 🇷🇺

Простая консольная утилита на Rust для записи звука с микрофона в WAV-файл (`recording.wav`).

## Возможности

- Автоматический выбор стандартного устройства ввода.
- Запись в формате WAV с плавающей точкой (F32).
- Корректная обработка завершения по Ctrl+C.

## Использование

1. **Соберите проект:**
   ```sh
   cargo build --release
   ```

2. **Запустите программу:**
   ```sh
   cargo run --release
   ```
   Запись начнётся автоматически, файл будет сохранён как `recording.wav` в текущей директории.

3. **Остановите запись** нажатием `Ctrl+C`.

## Зависимости

- [`cpal`](https://crates.io/crates/cpal) — работа с аудиоустройствами.
- [`hound`](https://crates.io/crates/hound) — запись WAV-файлов.
- [`ctrlc`](https://crates.io/crates/ctrlc) — обработка сигнала завершения.

## Примечания

- Требуется аудиоустройство с поддержкой формата F32.
- По умолчанию выбирается стандартное устройство ввода.

## Лицензия

MIT
