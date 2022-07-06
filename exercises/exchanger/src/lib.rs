use std::sync::{Mutex, Arc, Condvar};
use std::ops::DerefMut;

pub struct Exchanger<T> {
    shared_pair: Arc<ExchangerData<T>>,

}

struct ExchangerData<T> {
    mutex: Mutex<(Option<T>, Option<T>)>,
    cv_some: Condvar,
    cv_none: Condvar,
    exchange_in_progress: Mutex<bool>,
}

impl<T> Exchanger<T> {
    pub fn new() -> Self {
        Exchanger {
            shared_pair: Arc::new(
                ExchangerData {
                    mutex: Mutex::new((None, None)),
                    cv_none: Condvar::new(),
                    cv_some: Condvar::new(),
                    exchange_in_progress : Mutex::new(false),
                }
            )
        }
    }

    pub fn exchange(&self, x: T) -> T {
        let mut in_progrss = self.shared_pair.exchange_in_progress.lock().unwrap();
        while *in_progrss {
            in_progrss = self.shared_pair.cv_some.wait(in_progrss).unwrap();
        }

        let mut lock = self.shared_pair.mutex.lock().unwrap();
        let ret = if lock.0.is_some() {
            *in_progrss = true;
            lock.deref_mut().1 = Some(x);
            let received: Option<T> = std::mem::replace(&mut lock.deref_mut().0,None);
            self.shared_pair.cv_none.notify_all();
            received.unwrap()
        } else {
            lock.deref_mut().0 = Some(x);
            drop(in_progrss);
            while lock.1.is_none() {
                lock = self.shared_pair.cv_none.wait(lock).unwrap();
            }
            in_progrss = self.shared_pair.exchange_in_progress.lock().unwrap();
            *in_progrss = false;
            self.shared_pair.cv_some.notify_all();
            let take_res = std::mem::replace(&mut lock.deref_mut().1,None);
            take_res.unwrap()
        };

        return ret;
    }
}

impl<T> Clone for Exchanger<T>{
    fn clone(&self) -> Self {
        Exchanger{shared_pair: self.shared_pair.clone()}
    }
}