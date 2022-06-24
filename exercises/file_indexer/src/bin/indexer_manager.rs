//#![feature(is_some_with)]

use clap::{Parser, Arg};
use std::process::{exit, Stdio, Child, ChildStdin, ChildStdout};
use std::fs::{ReadDir};
use std::collections::{HashMap};
use std::io::{BufRead, BufReader, stdout, Write};
use std::iter::{Cycle, Enumerate};
use std::slice::{Iter};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{io, thread};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    folder: String,
    output: String,
}


fn main() {
    let args = clap::Command::new("File indexer")
        .author("Alessandro Mandrile")
        .version("0.1")
        .args([
            Arg::new("folder").index(1).required(true),
            Arg::new("output").short('f').long("output").takes_value(true)
        ])
        .get_matches();

    let folder = args.value_of("folder").unwrap();

    let output_file = args.value_of("output").unwrap_or("");
    let base_dir = if let Ok(res) = std::fs::read_dir(std::path::Path::new(folder)) {
        res
    } else {
        eprintln!("{} : Doesn't exist or it's not a directory", folder);
        exit(1);
    };

    let mut children: Vec<Child> = Vec::new();
    let mut stdin_pipes: Vec<ChildStdin> = Vec::with_capacity(children.len());
    let mut stdout_pipes: Vec<ChildStdout> = Vec::with_capacity(children.len());
    for _ in 0..8 {
        let mut child = std::process::Command::new("./indexer_worker.exe")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        stdin_pipes.push(child.stdin.take().unwrap());
        stdout_pipes.push(child.stdout.take().unwrap());
        children.push(child);
    }

    let (sender, receiver) = channel();
    let thread_producer = thread::spawn(move || {
        let mut cycle = stdin_pipes.iter().enumerate().cycle();
        depth_search(base_dir, &mut cycle, sender).expect("Depth search failed with err:");
    });

    let thread_consumer = thread::spawn(move || {
        retrieve_work(stdout_pipes, receiver)
    });

    thread_producer.join().unwrap();

    let indexes = thread_consumer.join().unwrap();
    // println!("{:?}",indexes);
    let opt_file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(output_file);
    let mut output: Box<dyn Write> = match opt_file {
        Ok(file) => Box::new(file),
        Err(e) => {
            eprintln!("Error in opening output file: {e}");
            Box::new(stdout())
        }
    };

    indexes.into_iter().for_each(|(key, val)| {
        write!(output, "{}:{}",key, val.total).expect("Error in final write");
        val.occurrences.into_iter().for_each(|(s,c)|{
            write!(output, " {s} {c}").expect("Error in final write");
        });
        writeln!(output).unwrap();
    });
}

fn depth_search(mut dir_it: ReadDir, children: &mut Cycle<Enumerate<Iter<ChildStdin>>>, sender: Sender<Message>) -> io::Result<()> {
    while let Some(entry) = dir_it.next() {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            depth_search(entry.path().read_dir().unwrap(), children, sender.clone())?;
        }

        let path = entry.path();

        let ext = match path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "",
        };

        if ext == "txt" {
            let file = match path.into_os_string().into_string() {
                Ok(file) => file,
                Err(_msg) => String::new()
            };
            run_child(file, children, sender.clone());
        }
    };

    // if let Err(msg) = sender.send(Message{child:-1,file:"".to_string()}) {
    //     println!("Error in depth_search termination: {}", msg);
    // }
    return Ok(());
}

//Write file\n so that child can read it
fn run_child(file: String, children_pipes: &mut Cycle<Enumerate<Iter<ChildStdin>>>, sender: Sender<Message>) -> () {
    //println!("Found: {file}");
    let (i, mut pipe) = children_pipes.next().unwrap();
    pipe.write_all(file.as_bytes()).expect("Failed to write to stdin");
    pipe.write_all("\n".as_bytes()).expect("Failed to write to stdin");
    if let Err(msg) = sender.send(Message { child: i, file }) {
        eprintln!("Error in run_child: {}", msg);
    }
}

fn retrieve_work(mut children_pipes: Vec<ChildStdout>, receiver: Receiver<Message>) -> HashMap<String, MapValue> {
    let mut indexes: HashMap<String, MapValue> = HashMap::new();
    while let Ok(child) = receiver.recv() {
        let pipe = &mut children_pipes[child.child];
        //println!("Received:{}", child.file);

        let mut buf = String::new();
        let mut done = false;
        let mut buffer = BufReader::new(pipe);

        //let mut lines = 0;
        while !done {
            let res = match buffer.read_line(&mut buf) {
                Ok(_n) => {
                    let mut res = String::from(&buf);
                    trim_newline(&mut res);
                    res
                }
                Err(_error) => {
                    String::new()
                }
            };

            //lines += 1;
            done = res == "";
            if !done {
                let (word, count) = parse_map_line(&res).unwrap_or(("".to_string(), 1));

                indexes.entry(word)
                    .and_modify(|mapped| { mapped.add(child.file.clone(), count); })
                    .or_insert(MapValue::new(child.file.clone(), count));
            }
            buf.clear();
        }
        //println!("Read {lines} lines from file ");
    }
    //println!("Retriever ended");
    indexes
}

fn parse_map_line(line: &str) -> Result<(String, u32), String> {
    let fields = line.split(':').collect::<Vec<&str>>();
    let word = match fields.first() {
        Some(word) => word.to_string(),
        None => return Err(("No word: ".to_string() + line).to_string())
    };

    let count_res = match fields.last() {
        Some(num) => num.parse::<u32>(),
        None => return Err(("No word: ".to_string() + line).to_string()),
    };

    let count = match count_res {
        Ok(num) => num,
        Err(e) => return Err(("No word: ".to_string() + e.to_string().as_str()).to_string()),
    };

    Ok((word, count))
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

struct Message {
    file: String,
    child: usize,
}

#[derive(Debug)]
struct MapValue {
    total: u32,
    occurrences: Vec<(String, u32)>,
}

impl MapValue {
    fn new(file: String, count: u32) -> Self {
        let vec = vec![(file, count)];
        MapValue { total: count, occurrences: vec }
    }
    fn add(&mut self, file: String, count: u32) -> () {
        self.total += count;
        self.occurrences.push((file, count));
    }
}



