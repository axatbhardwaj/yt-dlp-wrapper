# yt-dlp-wrapper

`yt-dlp-wrapper` is a Rust program that acts as a wrapper for `yt-dlp`, allowing you to download specific parts of YouTube videos by specifying start and end times. This is a sample project to explore command and process interactions in Rust.

## Features

- Download the best video and audio quality available.
- Specify start and end times in `HH:MM:SS` format.
- Downloads the video to the current directory.

## Prerequisites

- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [ffmpeg](https://ffmpeg.org/)
- [rustup](https://rustup.rs/)

Make sure `yt-dlp`, `ffmpeg`, and `rustup` are installed and accessible in your system's PATH.

### Installation on Windows

1. Install `yt-dlp` using pip:

   ```sh
   pip install yt-dlp
   ```

2. Install `ffmpeg` using winget:

   ```sh
   winget install --id=Gyan.FFmpeg  -e
   ```

3. Install `rustup`:
   ```sh
   winget install --id=Rustlang.Rustup -e
   ```

### Installation on Linux/MacOS

1. Install `yt-dlp` using pip:

   ```sh
   pip install yt-dlp
   ```

2. Install `ffmpeg` using your package manager, for example (Arch):

   ```sh
   paru -S ffmpeg
   ```

3. Install `rustup`:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/axatbhardwaj/yt-dlp-wrapper.git
   cd yt-dlp-wrapper
   ```

2. Build the project using Cargo:
   ```sh
   cargo build --release
   ```

## Usage

You can run the program with or without command-line arguments.

### With Command-Line Arguments

```sh
cargo run --release -- <URL> <start_time> <end_time>
```

### Without Command-Line Arguments

The program will prompt you to enter the URL, start time, and end time interactively.
