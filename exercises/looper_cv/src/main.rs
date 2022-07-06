use std::sync::mpsc::channel;
use looper_cv::looper::Looper;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (send,rec) = channel();
    let proc = move|i:i32|{send.send(i).unwrap();};
    let clean = ||{ sleep(Duration::from_secs(2));println!("I'm dead");};
    let looper = Looper::new(proc,clean);
    looper.send(1);
    looper.send(2);
    drop(looper);
    assert_eq!(rec.recv().unwrap(), 1);
    assert_eq!(rec.recv().unwrap(), 2);
}
