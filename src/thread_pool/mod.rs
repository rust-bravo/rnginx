use std::sync::mpsc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);
        let (sender, recevier) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
           workers,
           sender,
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                (*job)();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}
