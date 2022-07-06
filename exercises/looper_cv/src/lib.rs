pub mod looper {
    use std::thread;
    use std::sync::{Condvar, Mutex, Arc};
    use std::collections::VecDeque;

    pub struct Looper<M: Send> {
        ref_count: Arc<Mutex<u8>>,
        message_queue: Arc<Mutex<VecDeque<Option<M>>>>,
        has_msg: Arc<Condvar>,
    }

    impl<M: Send + 'static> Looper<M> {
        pub fn new(process_fn: impl Fn(M) -> () + Send + 'static,
                   cleanup_fn: impl FnOnce() -> () + Send + 'static) -> Self {
            let message_queue = Arc::new(Mutex::new(VecDeque::<Option<M>>::new()));
            let has_msg = Arc::new(Condvar::new());
            let ref_count = Arc::new(Mutex::new(1));

            let th_queue = Arc::clone(&message_queue);
            let th_cv = Arc::clone(&has_msg);

            thread::spawn(move || {
                let message_queue = th_queue;
                let has_msg = th_cv;
                loop {
                    let mut msg_lock = message_queue.lock().unwrap();
                    msg_lock = has_msg.wait_while(msg_lock, |lock| { lock.len() == 0 }).unwrap();
                    let msg_opt = msg_lock.pop_front().unwrap();
                    drop(msg_lock);
                    match msg_opt {
                        Some(msg) => process_fn(msg),
                        None => break,
                    }
                };
                cleanup_fn();
            });
            Self { message_queue, has_msg, ref_count }
        }

        pub fn send(&self, msg: M) {
            let mut queue_lock = self.message_queue.lock().unwrap();
            queue_lock.push_back(Some(msg));
            self.has_msg.notify_one();
        }
    }

    impl<M: Send> Clone for Looper<M> {
        fn clone(&self) -> Self {
            let mut refs = self.ref_count.lock().unwrap();
            *refs += 1;
            Self {
                ref_count: self.ref_count.clone(),
                has_msg: self.has_msg.clone(),
                message_queue: self.message_queue.clone(),
            }
        }
    }

    impl<M: Send> Drop for Looper<M> {
        fn drop(&mut self) {
            let mut refs = self.ref_count.lock().unwrap();
            *refs -=1;
            if *refs == 0 {
                let mut queue_lock = self.message_queue.lock().unwrap();
                queue_lock.push_back(None);
                self.has_msg.notify_one();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::looper::Looper;
        use std::sync::mpsc::{channel};
        use std::thread;

        #[test]
        fn dropping_looper_cause_the_thread_to_quit() {
            let (tx, rx) = channel::<i32>();
            let tx1 = tx.clone();
            let l = Looper::<i32>::new(
                move |m: i32| {
                    tx1.send(m).unwrap();
                },
                move || {
                    tx.send(1).unwrap();
                },
            );
            assert!(rx.try_recv().is_err());
            drop(l);
            assert_eq!(rx.recv().unwrap(), 1);
        }

        #[test]
        fn messages_are_processed_in_order() {
            let (tx, rx) = channel::<i32>();
            let tx1 = tx.clone();
            let tx2 = tx.clone();
            drop(tx);
            let l = Looper::<i32>::new(
                move |m: i32| {
                    tx1.send(m).unwrap();
                },
                move || {
                    tx2.send(3).unwrap();
                },
            );
            l.send(1);
            l.send(2);
            drop(l);
            assert_eq!(rx.recv().unwrap(), 1);
            assert_eq!(rx.recv().unwrap(), 2);
            assert_eq!(rx.recv().unwrap(), 3);
        }

        #[test]
        fn messages_sent_from_other_threads_work() {
            let (tx, rx) = channel::<i32>();
            let tx1 = tx.clone();
            let tx2 = tx.clone();
            let l = Looper::<i32>::new(
                move |m: i32| {
                    tx1.send(m).unwrap();
                },
                move || {
                    tx2.send(3).unwrap();
                },
            );
            let l1 = l.clone();
            thread::spawn(move || {
                l1.send(1);
            });
            let l1 = l.clone();
            thread::spawn(move || {
                l1.send(1);
            });
            drop(tx);
            drop(l);
            assert_eq!(rx.recv().unwrap(), 1);
            assert_eq!(rx.recv().unwrap(), 1);
            assert_eq!(rx.recv().unwrap(), 3);
        }
    }
}