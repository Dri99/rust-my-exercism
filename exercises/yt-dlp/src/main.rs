use std::process::Command;
use std::io::{BufReader, stdin, BufRead, stdout, Write};

fn main() {
    print!("Incolla qui il link e premi INVIO: ");
    stdout().flush().unwrap();
    let mut reader = BufReader::new(stdin());
    let mut url = String::new();
    reader.read_line(&mut url).unwrap();

    let trimmed = url.trim_end().to_string();
    let mut child = Command::new("./bin/yt-dlp.exe")
        .arg("--ffmpeg-location")
        .arg("bin/ffmpeg/bin")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(trimmed)
        .spawn()
        .unwrap();
    child.wait().unwrap();

}
