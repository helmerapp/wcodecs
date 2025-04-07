#![allow(unused)]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub const MAX_WORKERS: usize = 4;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap();
    }
}

/// Worker that runs in its own thread, pulling jobs from the shared queue.
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct WorkQueue {
    thread_pool: ThreadPool,
}

impl WorkQueue {
    pub fn new(num_threads: usize) -> Self {
        WorkQueue {
            thread_pool: ThreadPool::new(num_threads),
        }
    }

    pub fn enqueue(&self, task: Job) {
        self.thread_pool.execute(task);
    }
}
