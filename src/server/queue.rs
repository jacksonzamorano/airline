use super::{server::IncomingRequest, Response};
use std::{
    sync::{Mutex, mpsc::{Sender, channel, Receiver}, Arc},
    thread::{self, available_parallelism, JoinHandle}, io::Write,
};

pub struct RequestQueue {
    workers: Vec<RequestWorker>,
    sender: Option<Sender<IncomingRequest>>
}
impl RequestQueue {
    pub fn new() -> RequestQueue {
    	let (sender, reciever) = channel::<IncomingRequest>();
    	let wc = available_parallelism().unwrap().get();
        
    	let mut workers: Vec<RequestWorker> = Vec::new();

    	let rc_mutex = Arc::new(Mutex::new(reciever));

    	println!("Started with {} threads", &wc);

    	for _ in 0..wc {
    		workers.push(RequestWorker::spawn(rc_mutex.clone()));
    	}

    	RequestQueue { workers, sender: Some(sender) }
    }

    pub fn add(&self, ir: IncomingRequest) {
        _ = &self.sender.as_ref().unwrap().send(ir);
    }
}
impl Drop for RequestQueue {
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
	pub fn spawn(reciever: Arc<Mutex<Receiver<IncomingRequest>>>) -> RequestWorker {
		let thread = thread::spawn(move || {
			loop {
				let ir_task_op = reciever.lock().unwrap().recv();
				if let Ok(mut ir_task) = ir_task_op {
					// Create a new response
					let mut res = Response::new();
					// Minor validation
					
					// Tell the handler to parse it
					(ir_task.route)(&ir_task.request, &mut res);
					// Write stream
					let mut bytes = res.header();
					bytes.append(&mut res.bytes());
					_ = ir_task.stream.write(&bytes);
				} else {
					break;
				}
			}
		});

		RequestWorker { thread: Some(thread) }
	}
}