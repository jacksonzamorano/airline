use super::{server::IncomingRequest, Response};
use std::{
    io::Write,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{spawn, available_parallelism, JoinHandle},
};

pub struct RequestQueue<T: 'static + Send> {
    workers: Vec<RequestWorker>,
    sender: Option<Sender<IncomingRequest<T>>>,
}
impl<T: 'static + Send> RequestQueue<T> {
    pub fn new(f: WorkerSetupFn<T>) -> RequestQueue<T> {
        let (sender, reciever) = channel::<IncomingRequest<T>>();
        let wc = available_parallelism().unwrap().get() / 4;


        let mut workers: Vec<RequestWorker> = Vec::new();
        let rc_mutex = Arc::new(Mutex::new(reciever));

        println!("Started with {} threads", &wc);

        for _ in 0..wc {
            workers.push(RequestWorker::spawn(
                rc_mutex.clone(),
                f
            ));
        }

        RequestQueue {
            workers,
            sender: Some(sender),
        }
    }

    pub fn add(&self, ir: IncomingRequest<T>) {
        _ = &self.sender.as_ref().unwrap().send(ir);
    }
}
impl<T: 'static + Send> Drop for RequestQueue<T> {
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in &mut self.workers {
            if let Some(t) = w.thread.take() {
                _ = t.join();
            }
        }
    }
}

pub struct RequestWorker {
    thread: Option<JoinHandle<()>>,
}
impl RequestWorker {
    pub fn spawn<T: 'static + Send>(
        reciever: Arc<Mutex<Receiver<IncomingRequest<T>>>>,
        setup_fn: WorkerSetupFn<T>
    ) -> RequestWorker {
        let thread = spawn(move || {
            let data = setup_fn();
            loop {
                let ir_task_op = reciever.lock().unwrap().recv();
                if let Ok(mut ir_task) = ir_task_op {
                    // Create a new response
                    let mut res = Response::new();
                    // Minor validation
                    // Tell the handler to parse it
                    (ir_task.route)(&ir_task.request, &mut res, &data);
                    // Write stream
                    let mut bytes = res.header();
                    bytes.append(&mut res.bytes());
                    _ = ir_task.stream.write(&bytes);
                } else {
                    break;
                }
            }
        });

        RequestWorker {
            thread: Some(thread),
        }
    }
}

pub type WorkerSetupFn<T> = fn() -> T;