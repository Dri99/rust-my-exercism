use std::sync::Arc;
use cyclic_barrier::cb;

fn main() {
    let abarrrier = Arc::new(cb::CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}\n", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
