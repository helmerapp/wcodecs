use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::control::{self, ControlMessage, ControlMessageTrait, Outcome};

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

/// Internal slots shared by codec instances.
pub struct CodecInternalSlots {
    pub control_message_queue: VecDeque<control::ControlMessage>,
    pub message_queue_blocked: bool,
    pub work_queue: Arc<WorkQueue>,
}

impl CodecInternalSlots {
    pub fn new(num_threads: usize) -> Self {
        CodecInternalSlots {
            control_message_queue: VecDeque::new(),
            message_queue_blocked: false,
            work_queue: Arc::new(WorkQueue::new(num_threads)),
        }
    }

    /// Enqueue a control message and process the control message queue.
    pub fn enqueue_control_message(&mut self, msg: control::ControlMessage) {
        self.control_message_queue.push_back(msg);
        self.process_control_message_queue();
    }

    /// Sequential processing
    pub fn process_control_message_queue(&mut self) {
        while !self.message_queue_blocked && !self.control_message_queue.is_empty() {
            if let Some(front_msg) = self.control_message_queue.front_mut() {
                match front_msg {
                    ControlMessage::Decode(decode_msg) => {
                        let outcome = decode_msg.process();
                        match outcome {
                            Outcome::NotProcessed => break,
                            Outcome::Processed => {
                                self.control_message_queue.pop_front();
                            }
                        }
                    }
                    ControlMessage::Encode(encode_msg) => {
                        let outcome = encode_msg.process();
                        match outcome {
                            Outcome::NotProcessed => break,
                            Outcome::Processed => {
                                self.control_message_queue.pop_front();
                            }
                        }
                    }
                    ControlMessage::Config(config_msg) => {
                        let outcome = config_msg.process();
                        match outcome {
                            Outcome::NotProcessed => break,
                            Outcome::Processed => {
                                self.control_message_queue.pop_front();
                            }
                        }
                    }
                }
            }
        }
    }

    fn process_message<T: ControlMessageTrait>(&mut self, mut msg: T) -> bool {
        match msg.process() {
            Outcome::NotProcessed => false,
            Outcome::Processed => {
                self.control_message_queue.pop_front();
                true
            }
        }
    }
}
