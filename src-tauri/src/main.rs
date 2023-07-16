// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rusqlite;

use base64;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rusqlite::Connection;
use serde::Serialize;
// use cpal::{FromSample, Sample, SampleFormat};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

use screenshots::Screen;
use std::{fs, time::Instant};

extern crate image;
#[derive(Clone, serde::Serialize)]
struct TranscriptSegment {
    content: String,
    started_at: f64,
    ended_at: f64,
}

#[derive(Debug, Serialize)]
pub struct ContentRow {
    content: String,
    started_at: f64,
    ended_at: f64,
    source: String,
    id: i64, // add other fields as necessary
}

#[derive(Debug, Serialize)]
pub struct ScreenshotRow {
    image: Vec<u8>,
    source: String,
    id: i64, // add other fields as necessary
    created_at: f64,
    // optional field:
    base64_image: Option<String>,
}

const DATABASE_URL: &str = "mind_overflow.sqlite3";

lazy_static! {
    static ref AUDIO_TOGGLE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref AUDIO: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref SCREEN_TOGGLE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref SCREEN: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref DB_CONN: Arc<Mutex<Connection>> = Arc::new(Mutex::new(
        Connection::open(DATABASE_URL).expect("Unable to open SQLite database")
    ));
}

#[tauri::command]
fn screen_start(window: tauri::Window) {
    *SCREEN_TOGGLE.lock().unwrap() = true;

    println!(
        "Current screen state - toggle: {} - screen: {}",
        *SCREEN_TOGGLE.lock().unwrap(),
        *SCREEN.lock().unwrap()
    );

    if !*SCREEN.lock().unwrap() {
        std::thread::spawn(move || screen_process(window.clone()));
    }
}

#[tauri::command]
fn screen_stop() {
    *SCREEN_TOGGLE.lock().unwrap() = false;

    println!(
        "Current screen state - toggle: {} - screen: {}",
        *SCREEN_TOGGLE.lock().unwrap(),
        *SCREEN.lock().unwrap()
    );
}

fn screen_process(window: tauri::Window) {
    *SCREEN.lock().unwrap() = true;
    println!("Screen toggle");

    let screens = Screen::all().unwrap();
    let screen = screens.first().unwrap();

    // Start a loop, that will continue until SCREEN_TOGGLE is false
    while *SCREEN_TOGGLE.lock().unwrap() {
        let start = Instant::now();

        println!("capturer {screen:?}");

        let image = screen.capture().unwrap();
        let buffer = image.to_png().unwrap();
        // fs::write("../public/screenshot_chinois.png", buffer).unwrap();
        // println!("运行耗时: {:?}", start.elapsed());
        db_insert_screenshot(buffer).unwrap();
        db_get_screenshot(None, None, window.clone());

        // Delay for 5 seconds
        std::thread::sleep(Duration::from_secs(5));
    }

    *SCREEN.lock().unwrap() = false;
}

fn db_insert_screenshot(buffer: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();

    conn.execute(
        "INSERT INTO screenshots (image, source_type, source, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            &buffer,
            "screenshot",
            "desktop",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as f64
        ],
    )?;

    Ok(())
}

#[tauri::command]
fn audio_stop() {
    *AUDIO_TOGGLE.lock().unwrap() = false;
    println!(
        "Current audio state - toggle: {} - audio: {}",
        *AUDIO_TOGGLE.lock().unwrap(),
        *AUDIO.lock().unwrap()
    );
}

#[tauri::command]
fn audio_start(window: tauri::Window) {
    *AUDIO_TOGGLE.lock().unwrap() = true;

    println!(
        "Current audio state - toggle: {} - audio: {}",
        *AUDIO_TOGGLE.lock().unwrap(),
        *AUDIO.lock().unwrap()
    );

    if !*AUDIO.lock().unwrap() {
        std::thread::spawn(move || audio_process(window));
    }
}

fn audio_process(window: tauri::Window) {
    // Turn global variable audio to true
    *AUDIO.lock().unwrap() = true;

    println!("Starting stream");

    let (tx, rx) = mpsc::channel::<(Vec<f32>, Option<std::time::Instant>)>();

    let model_path = "models/ggml-small.bin";
    let model = WhisperContext::new(&model_path).expect("failed to load model");

    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("failed to get default input device");

    println!("Using default device: {}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    println!("Default input config: {:?}", config);

    let looping = Arc::new(Mutex::new(true));
    let looping_clone = Arc::clone(&looping);

    std::thread::spawn(move || {
        // audio_stream(config, device, tx).unwrap();
        // loop_audio_stream(config.clone(), &device, looping_clone, tx);
        while *looping_clone.lock().unwrap() {
            audio_stream(config.clone(), &device, tx.clone()).unwrap();
        }
    });

    std::thread::spawn(move || {
        for (buffer, start_time) in rx {
            // Call the transcription function with the received data
            let transcriptions = audio_transcript(&model, buffer, start_time).unwrap();

            for transcription in transcriptions {
                db_insert_transcript(&transcription);
                println!("Inserted transcript into database")
            }

            println!("Getting content");
            db_get_content(None, None, window.clone());
        }
    });

    while *AUDIO_TOGGLE.lock().unwrap() {
        std::thread::sleep(Duration::from_millis(1 * 1000));
    }

    // Looping false
    *looping.lock().unwrap() = false;
    *AUDIO.lock().unwrap() = false;

    println!("Stopping stream");
}

// Function capturing microphone input
// until there is silence for 5 seconds
const SILENCE_THRESHOLD: f32 = 0.055;

fn audio_stream(
    config: cpal::SupportedStreamConfig,
    device: &cpal::Device,
    tx: mpsc::Sender<(Vec<f32>, Option<std::time::Instant>)>,
) -> Result<(), anyhow::Error> {
    let buffer = Arc::new(Mutex::new(Vec::new())); // Define the buffer here
    let data = Arc::clone(&buffer);

    let start_time = std::time::Instant::now();

    let user_speaking = Arc::new(Mutex::new(true));
    let user_started = Arc::new(Mutex::new(None::<std::time::Instant>));
    let silence_start = Arc::new(Mutex::new(None::<std::time::Instant>));

    let user_started_clone = Arc::clone(&user_started);
    let user_speaking_clone = Arc::clone(&user_speaking);
    let silence_start_clone = Arc::clone(&silence_start);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |audio: &[f32], _: &cpal::InputCallbackInfo| {
                // Check if data is silence
                if audio.iter().all(|&x| x.abs() < SILENCE_THRESHOLD) {
                    let mut silence_start = silence_start_clone.lock().unwrap();

                    // Silence just started, record the start time
                    if silence_start.is_none() {
                        *silence_start = Some(std::time::Instant::now());
                    }

                    // If silence has lasted for more than 1 second, set user_speaking to false
                    if silence_start
                        .as_ref()
                        .map_or(false, |t| t.elapsed() >= std::time::Duration::from_secs(1))
                    {
                        *user_speaking_clone.lock().unwrap() = false;
                        // println!("Silence detected for more than 1 second");
                    }
                } else {
                    let mut silence_start = silence_start_clone.lock().unwrap();
                    *silence_start = None;

                    let mut user_speaking = user_speaking_clone.lock().unwrap();
                    *user_speaking = true;

                    let mut user_started = user_started_clone.lock().unwrap();
                    *user_started = Some(std::time::Instant::now());
                }

                if *user_started_clone.lock().unwrap() != None && *AUDIO_TOGGLE.lock().unwrap() {
                    let mut buffer = data.lock().unwrap();
                    buffer.push((audio.to_vec(), std::time::Instant::now())); // Accumulate the data with timestamp
                }
            },
            move |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?,
        _ => return Err(anyhow::Error::msg("Unsupported sample format")),
    };

    stream.play()?;

    while *user_speaking.lock().unwrap() || *user_started.lock().unwrap() == None {
        // Let recording go for roughly three seconds.
        println!("Listening");
        std::thread::sleep(std::time::Duration::from_millis(1 * 100));

        if start_time.elapsed() > std::time::Duration::from_secs(60) {
            println!("Buffer is more than 60 seconds long");
            break;
        }

        if *AUDIO_TOGGLE.lock().unwrap() == false {
            println!("Audio toggle is false");
            break;
        }
    }

    // Now process the accumulated data
    let mut buffer = buffer.lock().unwrap();
    let start_time = user_started.lock().unwrap();

    // Cut data from buffer that has been appended after last silence_start
    if let Some(silence_start) = *silence_start.lock().unwrap() {
        let last_valid_index = buffer
            .iter()
            .rposition(|&(_, t)| t <= silence_start)
            .unwrap_or(0);
        buffer.truncate(last_valid_index + 1); // Keep frames until silence
    }

    let buffer: Vec<f32> = buffer.iter().flat_map(|(audio, _)| audio.clone()).collect();

    tx.send((buffer, start_time.clone())).unwrap();
    println!("Sending audio data");
    Ok(())
}

fn audio_transcript(
    model: &WhisperContext,
    data: Vec<f32>,
    start_time: Option<std::time::Instant>,
) -> Result<Vec<TranscriptSegment>, anyhow::Error> {
    println!("audio received");
    let audio_data = whisper_rs::convert_stereo_to_mono_audio(&data).unwrap();

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

    // Set the number of threads to use to 1.
    // params.set_n_threads(1);
    // Enable translation.
    // params.set_translate(true);
    // Set the language to translate to to English.
    params.set_language(Some("fr"));
    // Disable anything that prints to stdout.
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let mut state = model.create_state().expect("failed to create state");

    state
        .full(params, &audio_data[..])
        .expect("failed to run model");

    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");

    let mut results = Vec::new();

    for i in 0..num_segments {
        let content = state
            .full_get_segment_text(i)
            .expect("failed to get segment");

        println!("content: {}", content);

        if let Some(start_time) = start_time {
            let now = std::time::SystemTime::now();
            let since_the_epoch = now
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap();

            let start_timestamp_in_secs = state
                .full_get_segment_t0(i)
                .expect("failed to get segment start timestamp");
            let end_timestamp_in_secs = state
                .full_get_segment_t1(i)
                .expect("failed to get segment end timestamp");

            let started_at =
                since_the_epoch.as_secs() as f64 * 1000.0 + start_timestamp_in_secs as f64 * 1000.0;
            let ended_at =
                since_the_epoch.as_secs() as f64 * 1000.0 + end_timestamp_in_secs as f64 * 1000.0;

            results.push(TranscriptSegment {
                content,
                started_at,
                ended_at,
            });
        }
    }

    Ok(results)
}

#[tauri::command]
fn db_get_content(before: Option<f64>, after: Option<f64>, window: tauri::Window) {
    let conn = DB_CONN.lock().unwrap();

    let before_time = match before {
        Some(time) => time,
        None => std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64,
    };

    let after_time = after.unwrap_or(0.0);

    let mut stmt = conn
        .prepare(
            "SELECT id, content, started_at, ended_at, source FROM content WHERE created_at <= ? AND created_at > ?
             ORDER BY created_at DESC",
        )
        .unwrap();

    let rows = stmt
        .query_map(params![before_time, after_time], |row| {
            Ok(ContentRow {
                id: row.get(0)?,
                content: row.get(1)?,
                started_at: row.get(2)?,
                ended_at: row.get(3)?,
                source: row.get(4)?,
            })
        })
        .unwrap();

    let mut results = Vec::new();
    for row_result in rows {
        match row_result {
            Ok(row_data) => results.push(row_data),
            Err(err) => eprintln!("Database error: {}", err),
        }
    }

    window
        .emit("content", &results)
        .expect("Failed to emit event");
}

#[tauri::command]
fn db_get_screenshot(before: Option<f64>, after: Option<f64>, window: tauri::Window) {
    let conn = DB_CONN.lock().unwrap();

    let before_time = match before {
        Some(time) => time,
        None => std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64,
    };

    let after_time = after.unwrap_or(0.0);

    let mut stmt = conn
        .prepare(
            "SELECT id, image, created_at, source FROM screenshots WHERE created_at <= ? AND created_at > ?
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .unwrap();

    let rows = stmt
        .query_map(params![before_time, after_time], |row| {
            Ok(ScreenshotRow {
                id: row.get(0)?,
                image: row.get(1)?,
                created_at: row.get(2)?,
                source: row.get(3)?,
                // cast row.get(1) to type Vec<u8> then encode to base64
                base64_image: Some(base64::encode(row.get::<_, Vec<u8>>(1)?)),
            })
        })
        .unwrap();

    let mut results = Vec::new();
    for row_result in rows {
        match row_result {
            Ok(row_data) => results.push(row_data),
            Err(err) => eprintln!("Database error: {}", err),
        }
    }

    window
        .emit("screenshot", &results)
        .expect("Failed to emit event");
}

fn db_insert_transcript(transcript: &TranscriptSegment) -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();

    conn.execute(
        "INSERT INTO content (content, context, source_type, source, created_at, started_at, ended_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&transcript.content, Option::<String>::None, "audio", "microphone", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64, transcript.started_at, transcript.ended_at],
    )?;

    Ok(())
}

fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    let conn = DB_CONN.lock().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS content (
            id INTEGER PRIMARY KEY,

            content TEXT NOT NULL,
            context TEXT,

            embedding BLOB,

            source_type TEXT,
            source TEXT,
            
            created_at REAL NOT NULL,

            started_at REAL,
            ended_at REAL
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
            id INTEGER PRIMARY KEY,

            image BLOB NOT NULL,
            embedding BLOB,

            source_type TEXT,
            source TEXT,
            
            created_at REAL NOT NULL
        )",
        params![],
    )?;

    // Create index on created_at DESC and ASC
    conn.execute(
        "CREATE INDEX IF NOT EXISTS content_created_at_desc ON content (created_at DESC)",
        params![],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS content_created_at_asc ON content (created_at ASC)",
        params![],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS screenshots_created_at_desc ON screenshots (created_at DESC)",
        params![],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS screenshots_created_at_asc ON screenshots (created_at ASC)",
        params![],
    )?;

    Ok(())
}

fn main() {
    initialize().expect("Failed to initialize application");

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            audio_start,
            audio_stop,
            db_get_content,
            db_get_screenshot,
            screen_start,
            screen_stop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
