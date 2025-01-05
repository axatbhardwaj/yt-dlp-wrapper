use std::env;
use std::io::{self, Write};
use std::process::Command;

fn time_to_seconds(time_str: &str) -> Result<i32, &'static str> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return Err("Time format should be HH:MM:SS");
    }
    let hours: i32 = parts[0].parse().map_err(|_| "Invalid hours")?;
    let minutes: i32 = parts[1].parse().map_err(|_| "Invalid minutes")?;
    let seconds: i32 = parts[2].parse().map_err(|_| "Invalid seconds")?;
    Ok(hours * 3600 + minutes * 60 + seconds)
}

fn check_command_installed(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .is_ok()
}

fn get_video_duration(url: &str) -> Result<i32, &'static str> {
    let output = Command::new("yt-dlp")
        .arg("--get-duration")
        .arg(url)
        .output()
        .map_err(|_| "Failed to get video duration")?;
    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    time_to_seconds(&duration_str)
}

fn download_video(url: &str, start_time: Option<&str>, end_time: Option<&str>) {
    let video_duration = get_video_duration(url).expect("Failed to get video duration");

    let start_seconds = match start_time {
        Some(time) => time_to_seconds(time).expect("Invalid start time format"),
        None => 0,
    };

    let end_seconds = match end_time {
        Some(time) => time_to_seconds(time).expect("Invalid end time format"),
        None => video_duration,
    };

    if start_seconds >= video_duration {
        eprintln!("Error: Start time is out of bounds.");
        std::process::exit(1);
    }

    if end_seconds > video_duration {
        eprintln!("Error: End time is out of bounds.");
        std::process::exit(1);
    }

    let args = format!("ffmpeg_i:-ss {} -to {}", start_seconds, end_seconds);

    Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo+bestaudio")
        .arg("--external-downloader")
        .arg("ffmpeg")
        .arg("--external-downloader-args")
        .arg(args)
        .arg(url)
        .status()
        .expect("Failed to execute yt-dlp");
}

fn main() {
    if !check_command_installed("yt-dlp") {
        eprintln!("Error: yt-dlp is not installed or not found in PATH.");
        std::process::exit(1);
    }

    if !check_command_installed("ffmpeg") {
        eprintln!("Error: ffmpeg is not installed or not found in PATH.");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut input = String::new();
        print!("Enter URL: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let start_time = if args.len() > 2 {
        Some(args[2].as_str())
    } else {
        let mut input = String::new();
        print!("Enter start time (HH:MM:SS) or press Enter to download from the beginning: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    };

    let end_time = if args.len() > 3 {
        Some(args[3].as_str())
    } else {
        let mut input = String::new();
        print!("Enter end time (HH:MM:SS) or press Enter to download until the end: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    };

    download_video(&url, start_time, end_time);
}