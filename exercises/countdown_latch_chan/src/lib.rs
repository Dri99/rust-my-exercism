use std::sync::{Mutex, Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};


pub struct CountDownLatch {
    count : Arc<Mutex<u8>>,
    locked: Arc<Mutex<u8>>,
    receiver: Arc<Mutex<Receiver<()>>>,
    sender: Sender<()>,
}

impl CountDownLatch {
    pub fn new(count: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<()>();
        CountDownLatch {
            count :Arc::new(Mutex::new(count)),
            locked: Arc::new(Mutex::new(0)),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn wait(&self) {

        let mut locked = self.locked.lock().unwrap();
        *locked += 1;
        drop(locked);

        let count = self.count.lock().unwrap();
        if *count == 0 {
            return;
        }

        let rec = self.receiver.lock().unwrap();
        rec.recv().unwrap();
    }

    pub fn count_down(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        if *count == 0 {
            let locked = self.locked.lock().unwrap();
            for _ in 0..*locked {
                self.sender.send(()).unwrap();
            }
        }
    }
}