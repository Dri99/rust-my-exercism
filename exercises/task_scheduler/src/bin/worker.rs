use std::{error, fs, io, process, thread, time};
use std::io::{BufRead, Write};
use std::str::FromStr;
use clap::Arg;
use task_scheduler::scheduler_message::{Message, MessageReader};

fn main() {
    let args = clap::Command::new("Parallel Task worker")
        .author("Alessandro Mandrile")
        .version("0.1")
        .args([
            Arg::new("task_file").index(1).required(true),
        ])
        .get_matches();

    let file_name = match args.value_of("task_file") {
        Some(file) => file,
        None => {
            eprintln!("Missing required argument task_file");
            process::exit(1);
        }
    };

    let mut done = false;
    let mut reader = MessageReader::new(io::stdin());
    while !done {
        println!("{}", Message::LockReq);
        let response = reader.read();
        match response {
            Message::Done => process::exit(0),
            Message::Err(e) => {
                eprintln!("Received from scheduler unexpected {}, exiting", e);
                process::exit(1);
            }
            Message::LockGrant => {
                if let Err(e) = run_task(file_name, &mut done) {
                    done = true;
                    println!("{}", Message::Err(e.to_string()));
                };
            }
            _ => {
                eprintln!("Should not receive Message:{response}, will continue");
            }
        }
    }
}

fn run_task(task_file: &str, done: &mut bool) -> Result<(), Box<dyn error::Error>> {

    let fp = fs::File::open(task_file).expect(format!("Cannot open task file:{}",task_file).as_str());
    let mut reader = io::BufReader::new( fp );

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let tmp_file_string = format!("{}.tmp",task_file);
    let tmp_file_name:&str = tmp_file_string.as_str();
    let tmp_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(tmp_file_name).expect(format!("Cannot open file tmp:{}",tmp_file_name).as_str());

    let mut writer = io::BufWriter::new(tmp_file);

    io::copy(&mut reader, &mut writer)?;

    writer.flush()?;
    drop(writer);

    fs::remove_file(task_file).unwrap();
    fs::rename(tmp_file_name, task_file).expect(format!("Cannot rename {} to {}",tmp_file_name,task_file).as_str());
    println!("{}", Message::LockRelease);

    if first_line.trim_end().len() == 0 {
        *done = true;
        println!("{}",Message::Done);
        return Ok(());
    }

    let fields : Vec<String>= first_line.trim_end().split_whitespace().map(|str|str.to_string()).collect();

    if fields.len() != 2 {
        return Err(Box::new(Message::Err(format!("Bad line format: {}", first_line))));
    }

    let time:u64 = u64::from_str(fields[1].as_str())?;


    thread::sleep(time::Duration::from_secs(time));
    Ok(())
}
