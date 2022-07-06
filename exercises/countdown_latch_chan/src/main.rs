use countdown_latch_chan::CountDownLatch;
use std::time::Duration;

fn main() {
    let cdl = CountDownLatch::new(1);
    let cdl2 = cdl.clone();

    let th = std::thread::spawn(move|| {
        std::thread::sleep(Duration::new(2,0));
        println!("Initialise done");
        cdl2.count_down();
        println!("Thread end");
    });
    std::thread::sleep(Duration::new(4,0));
    cdl.wait();
    println!("Main: thread unlocked");
    th.join().unwrap();

}
