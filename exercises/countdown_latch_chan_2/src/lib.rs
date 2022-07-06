use std::sync::mpsc::{channel, sync_channel, SyncSender, Sender, Receiver};
use std::sync::{Mutex, Arc};

//#[derive(Clone)]
pub struct CountdownLatch {
    count : usize,
    waiting_sender: Mutex<SyncSender<()>>,
    counting_sender: Mutex<Sender<()>>,
    waiting_receiver: Arc<Mutex<Option<Receiver<()>>>>,
    counting_receiver: Arc<Mutex<Option<Receiver<()>>>>,
}

impl CountdownLatch {
    pub fn new(count: usize) -> Self {
        let (counting_sender, counting_receiver) = channel();
        let (waiting_sender, waiting_receiver) = sync_channel(0);

        Self {
            count,
            waiting_sender: Mutex::new(waiting_sender),
            counting_sender: Mutex::new(counting_sender),
            counting_receiver: Arc::new(Mutex::new(Some(counting_receiver))),
            waiting_receiver: Arc::new(Mutex::new(Some(waiting_receiver))),
        }
    }

    pub fn wait(&self) {
        //The first thread waiting will do the job of the companion thread,
        // the others will wait for the send to be retrieved
        let mut counting_receiver_lock = self.counting_receiver.lock().unwrap();
        if counting_receiver_lock.is_some(){
            let counting_receiver = std::mem::take(&mut *counting_receiver_lock).unwrap();
            drop(counting_receiver_lock);

            for _ in 0..self.count {
                counting_receiver.recv().unwrap();
            }

            let mut waiting_receiver_lock = self.waiting_receiver.lock().unwrap();
            let waiting_receiver = std::mem::take(&mut *waiting_receiver_lock).unwrap();
            while waiting_receiver.try_recv().is_ok() {}
        } else {
            drop(counting_receiver_lock);
            let sender_lock = self.waiting_sender.lock().unwrap();
            let sender = sender_lock.clone();
            drop(sender_lock);
            sender.send(()).unwrap_or(());
        }
    }

    pub fn count_down(&self) {
        let counting_sender = self.counting_sender.lock().unwrap();
        counting_sender.send(()).unwrap_or(());
    }
}