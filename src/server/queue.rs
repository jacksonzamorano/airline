use super::{IncomingRequest, Response};
use std::{
    io::Write,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{available_parallelism, spawn, JoinHandle},
};

pub struct RequestQueue<T: 'static + Send> {
    workers: Vec<RequestWorker>,
    sender: Option<Sender<IncomingRequest<T>>>,
}
impl<T: 'static + Send> RequestQueue<T> {
    pub fn new(f: WorkerSetupFn<T>) -> RequestQueue<T> {
        let wc = available_parallelism().unwrap().get() / 2;
        RequestQueue::new_with_thread_count(f, wc)
    }

    pub fn new_with_thread_count(f: WorkerSetupFn<T>, thread_count: usize) -> RequestQueue<T> {
        let (sender, reciever) = channel::<IncomingRequest<T>>();
        let mut workers: Vec<RequestWorker> = Vec::new();
        let rc_mutex = Arc::new(Mutex::new(reciever));

        println!("Started with {} threads", &thread_count);

        for _ in 0..thread_count {
            workers.push(RequestWorker::spawn(rc_mutex.clone(), f));
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
        setup_fn: WorkerSetupFn<T>,
    ) -> RequestWorker {
        let thread = spawn(move || {
        	// Spawn the required helper object
            let data = setup_fn();
            loop {
                let ir_task_op = reciever.lock().unwrap().recv();
                if let Ok(mut ir_task) = ir_task_op {
                    // Create a new response
                    let mut res = Response::new();
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
