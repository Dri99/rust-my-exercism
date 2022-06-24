use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::string::String;


fn main() {
    let stdin = stdin();
    let mut locked = stdin.lock();
    let mut buf = String::new();

    //println!("Started");
    let mut done = false;
    while !done {
        let mut map :HashMap<String,i32> = HashMap::new();
        let res = match locked.read_line(&mut buf){
            Ok(n) => {
                //println!("Read {n} bytes");
                done = n == 0;
                String::from(&buf).trim().to_string()
            },
            Err(_error) => {
                //println!("error: {error}");
                done = true;
                String::new()
            },
        };

        let file = if let Ok(file) = File::open(res){
            file
        } else {
            continue;
        };
        let buffered = BufReader::new(file);

        for line in buffered.lines() {
            for word in line.unwrap_or("".to_string()).split_whitespace(){
                let str = word.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                map.entry(str)
                    .and_modify(|e| { *e += 1 })
                    .or_insert(1);
            }
        }
        for (k,v) in map{
            println!("{k}:{v}");
        }
        println!();

        buf.clear();
    }
    //println!("Done");
}