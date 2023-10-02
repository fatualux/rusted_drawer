use clap::Parser;
use kbar::Bar;
use rodio::{Decoder, OutputStream, Sink};
use std::io::{self, Write};
use std::io::BufReader;
use std::io::Cursor;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Prompt the user for the time interval and volume inputs
    let interval_input = get_user_input("Enter the time interval (hh.mm.ss format): ");
    let volume_input = get_user_input("Enter the volume (0.0-1.0): ");

    // Parse the command line arguments
    let args = Args {
        time: interval_input,
        volume: f32::from_str(&volume_input).unwrap_or(0.5), // Default to 0.5 if invalid input
    };

    // Calculate the sleep time based on the time interval
    let sleep_time = hms_to_seconds(&args.time) / 100.0;

    // Create a new progress bar
    let mut bar = Bar::new();
    bar.set_job_label(format!("Timer {} :", args.time).as_str());

    // Update the progress bar and sleep for the specified interval
    for x in 1..=100 {
        bar.reach_percent(x);
        sleep(Duration::from_secs_f64(sleep_time));
    }

    println!("Finished 🥳️");

    // Play the alert sound
    let wav_file = include_bytes!("sounds/alert.wav");
    let wav_file = Cursor::new(wav_file);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(wav_file);
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_volume(args.volume);
    sink.sleep_until_end();
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "Use time in hh.mm.ss format")]
    time: String,

    #[arg(default_value = "0.5", help = "Volume of alert sound (0.0-1.0)")]
    volume: f32,
}

fn hms_to_seconds(s: &str) -> f64 {
    let parts: Vec<f64> = s.split('.').map(|part| f64::from_str(part).unwrap()).collect();
    parts[0] * 3600.0 + parts[1] * 60.0 + parts[2]
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
