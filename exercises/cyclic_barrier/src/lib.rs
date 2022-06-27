pub mod cb {
    use std::cell::Cell;
    use std::sync::{Arc, Condvar, Mutex};

    pub struct CyclicBarrier {
        count: u8,
        //in_barrier: Arc<u8>,
        //emptying : Mutex<bool>,
        //cv: Condvar,
        //shared: Arc<SharedData>,
        cv_enter: Condvar,
        cv_exit: Condvar,
        filling: Mutex<bool>,
        //emptying: Mutex<bool>,
        in_barrier: u8,
    }

    struct SharedData {
        cv_enter: Condvar,
        cv_exit: Condvar,
        filling: Mutex<bool>,
        //emptying: Mutex<bool>,
        in_barrier: Cell<u8>,
    }

    impl CyclicBarrier {

        pub fn new(count: u8) -> Self {
            CyclicBarrier {
                count,
                cv_enter: Condvar::new(),
                cv_exit: Condvar::new(),
                filling: Mutex::new(true),
                in_barrier: 0,
            }
        }

        pub fn wait(& self) -> () {
            let mut filling = self.filling.lock().unwrap();

            if self.in_barrier == 0 {
                *filling = true;
                self.cv_enter.notify_all();
            }
            //in barrier for real
            while !*filling {
                filling = self.cv_enter.wait(filling).unwrap();
            }

            //let mut emptying = self.shared.emptying.lock().unwrap();
            //release filling
            self.in_barrier += 1;

            if self.in_barrier == self.count {
                *filling = false;
                self.cv_exit.notify_all();
            }
            while *filling {
                //first sleeping holds also filling, np since nobody should enter anyway
                filling = self.cv_exit.wait(filling).unwrap();
            }
            self.in_barrier -= 1;

            if self.in_barrier == 0 {
                *filling = true;
                self.cv_enter.notify_all();
            }
        }
    }

    // impl Clone for CyclicBarrier {
    //     fn clone(&self) -> Self {
    //         CyclicBarrier { count: self.count, shared: self.shared.clone() }
    //     }
    // }
}
