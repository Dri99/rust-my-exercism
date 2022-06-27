use clap::{Arg, Parser};
use std::collections::VecDeque;
use std::io::Write;
use std::process::{ChildStdin, Stdio};
use std::{fs, process, sync::mpsc, thread};
use task_scheduler::scheduler_message;
use task_scheduler::scheduler_message::Message;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    task_file: String,
    child_num: u8,
}

fn main() {
    let args = clap::Command::new("Parallel Task scheduler")
        .author("Alessandro Mandrile")
        .version("0.1")
        .args([
            Arg::new("task_file").index(1).required(true),
            Arg::new("child_num")
                .long("child_num")
                .short('n')
                .value_parser(clap::value_parser!(u8))
                .default_value("2"),
        ])
        .get_matches();

    let file_name = match args.value_of("task_file") {
        Some(file) => file,
        None => {
            eprintln!("Missing required argument task_file");
            process::exit(1);
        }
    };

    match fs::File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Cannot open tas file: {}", e);
            process::exit(2);
        }
    };

    let procs = match args.get_one::<u8>("child_num") {
        Some(num) => *num,
        None => {
            eprintln!("Error in parsing child_num, using 2");
            2
        }
    };

    let mut stdin_pipes = Vec::with_capacity(procs as usize);
    let mut threads = Vec::with_capacity(procs as usize);

    let (send, rec) = mpsc::channel::<(Message, u8)>();

    for i in 0..procs {
        let mut child = process::Command::new("./worker.exe")
            .arg(file_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Error starting child with cmd()").as_str());

        stdin_pipes.push(child.stdin.take().unwrap());

        let stdout = child.stdout.take().unwrap();
        let sender = send.clone();
        threads.push(thread::spawn(move || {
            scheduler_thread(i, sender, stdout);
        }));
    }

    let mut done = false;
    let mut lock_owner = -1;
    let mut requests: VecDeque<u8> = VecDeque::new();
    while !done {
        if requests.len() != 0 && lock_owner == -1 {
            //I can grant one request
            let from = requests.pop_front().unwrap();
            lock_owner = from as i32;
            // let msg_for_child =  format!("{}",Message::LockGrant);
            // let stdin = &stdin_pipes[from];
            writeln!(stdin_pipes[from as usize], "{}", Message::LockGrant).unwrap();
            // stdin_pipes[from].write(msg_for_child.as_bytes()).unwrap();
        }

        let msg = match rec.recv() {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("All threads are dead: {}", e);
                process::exit(2);
            }
        };

        match msg.0 {
            Message::LockReq => requests.push_back(msg.1),
            Message::LockRelease => {
                if lock_owner == msg.1 as i32 {
                    lock_owner = -1;
                } else {
                    panic!(
                        "Lock release from unexpected worker:{}, was of {}",
                        msg.1, lock_owner
                    );
                }
            }
            Message::Done => {
                manage_done(msg.1, &mut stdin_pipes);
                done = true;
            }
            _ => eprintln!("Received unexpected msg from {}:{}", msg.1, msg.0),
        };
        println!("Received {} from {}", msg.0, msg.1);
    }
    println!("Received one done, waiting for all to finish");
    for thread in threads{
        thread.join().unwrap();
    }
}

fn scheduler_thread(
    nth: u8,
    sender: mpsc::Sender<(Message, u8)>,
    stdout: process::ChildStdout,
) -> () {
    let mut reader = scheduler_message::MessageReader::new(stdout);
    let mut done = false;

    while !done {
        let to_send = reader.read();
        done = to_send == Message::Done;
        if let Message::Err(_str) = &to_send {
            done = true;
        }
        if let Err(e) = sender.send((to_send, nth)) {
            eprintln!("Error in channel-{nth}: {}, closing connection", e);
            done = true;
        };
    }
    println!("Thread {nth} done");
}

fn manage_done(_exclude: u8, stdins: &mut Vec<ChildStdin>) -> () {
    stdins.iter_mut().for_each(|stdin| {
        writeln!(stdin, "{}", Message::Done).unwrap_or_default();
    });
}
