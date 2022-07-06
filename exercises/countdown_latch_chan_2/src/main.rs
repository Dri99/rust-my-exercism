use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use countdown_latch_chan_2::CountdownLatch;
fn main() {
    let cdl = Arc::new(CountdownLatch::new(2));
    let cdl1 = cdl.clone();
    let cdl2 = cdl.clone();
    let t1 = thread::spawn(move ||{
        sleep(Duration::from_secs(2));
        println!("Thread1 init done");
        cdl1.count_down();
        sleep(Duration::from_secs(5));
        println!("Thread1 done");
    });
    let t2 = thread::spawn(move ||{
        sleep(Duration::from_secs(3));
        println!("Thread2 init done");
        cdl2.count_down();
        sleep(Duration::from_secs(6));
        println!("Thread2 done");
    });

    cdl.wait();
    println!("Done");
    t1.join().unwrap();
    t2.join().unwrap();
}
