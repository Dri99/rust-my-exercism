pub mod cb {
    use std::sync::{Condvar, Mutex, RwLock};

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
        in_barrier: RwLock<u8>,
    }

    // struct SharedData {
    //     cv_enter: Condvar,
    //     cv_exit: Condvar,
    //     filling: Mutex<bool>,
    //     //emptying: Mutex<bool>,
    //     in_barrier: Cell<u8>,
    // }

    impl CyclicBarrier {

        pub fn new(count: u8) -> Self {
            CyclicBarrier {
                count,
                cv_enter: Condvar::new(),
                cv_exit: Condvar::new(),
                filling: Mutex::new(true),
                in_barrier: RwLock::new(0),
            }
        }

        pub fn wait(& self) -> () {
            let mut filling = self.filling.lock().unwrap();

            let in_barrier_r = self.in_barrier.read().unwrap();
            if  *in_barrier_r == 0 {
                *filling = true;
                self.cv_enter.notify_all();
            }
            drop(in_barrier_r);
            //in barrier for real
            while !*filling {
                filling = self.cv_enter.wait(filling).unwrap();
            }

            let mut in_barrier_w =self.in_barrier.write().unwrap();
            *in_barrier_w += 1;
            if *in_barrier_w == self.count {
                *filling = false;
                self.cv_exit.notify_all();
            }
            drop(in_barrier_w);

            while *filling {
                //first sleeping holds also filling, np since nobody should enter anyway
                filling = self.cv_exit.wait(filling).unwrap();
            }

            in_barrier_w =self.in_barrier.write().unwrap();
            *in_barrier_w -= 1;

            if *in_barrier_w == 0 {
                *filling = true;
                self.cv_enter.notify_all();
            }
         }
     }

    // impl Clone for CyclicBarrier {
    //     fn clone(&self) -> Self {
    //         CyclicBarrier { count: self.count, shared: self.shared.clone() }
    //     }
    //}
}
