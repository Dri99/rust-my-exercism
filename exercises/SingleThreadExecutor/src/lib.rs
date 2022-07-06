/*La classe SingleThreadExecutor implementa il concetto di ThreadPool basato su un singolo
thread incapsulato all'interno di ciascuna sua istanza. In assenza di richieste di lavoro, tale thread
resta fermo senza consumare cicli macchina.
Attraverso il metodo submit(...) è possibile affidare ad un'istanza di SingleThreadExecutor un
compito da eseguire. Tale compito viene inizialmente accodato e, non appena il thread
incapsulato è libero, viene eseguito.
Attraverso il metodo close() è possibile impedire l'ulteriore accodamento di compiti (eventuali
tentativi di invocare submit(...) dopo la chiamata a close() origineranno un'eccezione).
Attraverso il metodo join() è possibile attendere che tutti i compiti ancora da svolgere siano svolti e
il thread incapsulato nell'istanza termini.
Si implementi tale classe usando le funzionalità offerte dalla libreria C++11, definendo tutte le parti
eventualmente mancanti nella definizione della classe.
Si faccia attenzione al fatto che il codice che può essere sottomesso all'esecutore è arbitrario e
può contenere richieste di sottomissione di ulteriori compiti allo stesso esecutore.
class SingleThreadExecutor {
public:
void submit(std::packaged_task<void()> t);
//invia un compito da eseguire o lancia un'eccezione se l'
istanza è chiusa
void close();
//impedisce la sottomissione di compiti ulteriori, permettendo la terminazione del thread
incapsulato
void join();
//attende la terminazione del thread incapsulato
};
*/

use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, Condvar};

#[derive(Clone)]
pub struct SingleThreadExecutor {
    shared: Arc<Mutex<SharedData>>,
    sender: Sender<ThreadMsg>,
}

struct SharedData {
    executor: Option<std::thread::JoinHandle<()>>,
    closing: bool,
}

enum ThreadMsg {
    Task(Box<dyn Fn() -> () + Send>),
    Done,
    //None,
}

impl SingleThreadExecutor {
    fn thread_function(receiver: Receiver<ThreadMsg>) {
        loop {
            let task = receiver.recv().unwrap();
            match task {
                ThreadMsg::Done => break,
                ThreadMsg::Task(task_box) => {
                    task_box();
                }
            };
        }
    }

    pub fn new() -> Self {
        let (sender, rec) = channel();
        let executor = Some(std::thread::spawn(move || {
            SingleThreadExecutor::thread_function(rec);
        }));
        let shared = Arc::new(Mutex::new(SharedData { executor, closing: false }));
        SingleThreadExecutor { sender, shared }
    }

    pub fn submit(&self, task: Box<dyn Fn() -> () + Send>) -> Result<(), String> {
        let lock = self.shared.lock().unwrap();
        if !lock.closing {
            self.sender.send(ThreadMsg::Task(task)).unwrap();
        } else {
            return Err("The instance is closed or closing, cannot submit".to_string());
        }
        Ok(())
    }

    pub fn close(&self) {
        let mut lock = self.shared.lock().unwrap();
        if !lock.closing {
            self.sender.send(ThreadMsg::Done).unwrap();
            lock.closing = true;
        }
    }

    pub fn join(self) {
        let cv = Condvar::new();
        let mut lock = self.shared.lock().unwrap();
        lock = cv.wait_while(lock, |lock| { !lock.closing }).unwrap();

        let join_handle = std::mem::take(&mut lock.executor);
        match join_handle {
            Some(handle) => handle.join().unwrap(),
            None => unreachable!(),
        }
    }
}

