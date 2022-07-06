use std::sync::{Mutex, Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
pub struct CountDownLatch {
    count: Arc<Mutex<u8>>,
    receiver: Arc<Mutex<Receiver<()>>>,
    sender: Arc<Mutex<Option<Sender<()>>>>,
}

impl CountDownLatch {
    pub fn new(count: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<()>();
        CountDownLatch {
            count: Arc::new(Mutex::new(count)),
            sender: Arc::new(Mutex::new(Some(sender))),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn wait(&self) {
        let rec = self.receiver.lock().unwrap();
        rec.recv().unwrap_or(());
    }

    pub fn count_down(&self) {
        let mut count = self.count.lock().unwrap();
        if *count == 0 {
            return;
        }
        *count -= 1;
        if *count == 0 {
            let mut sender_lock = self.sender.lock().unwrap();
            *sender_lock = None;
        }
    }
}