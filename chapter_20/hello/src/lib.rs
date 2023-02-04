use chrono::prelude::*;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// ThreadPool 
/// 
/// # Type `Struct`
/// 
/// ## Fields
/// `workers` == Vec<Worker>
/// `sender` ==Option<mpsc::Sender<Job>>
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    time_alive: DateTime<Utc>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let date = Utc::now();

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender), time_alive: date }
    }

    /// Send a job type of Box<dyn FnOnce() + Send + 'static> to the receiver
    /// 
    /// It doesn't return anything
    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("The threadPool was running for {} minutes", (Utc::now().timestamp() - self.time_alive.timestamp()) / 60)
    }
}

impl Worker {
    /// Create a new Worker
    ///
    /// The id of the worker is received as a parameter
    ///     
    /// # Note
    /// If the operating system can’t create a thread because there aren’t enough system resources, thread::spawn will panic. That will cause our whole server to panic, even though the creation of some threads might succeed. For simplicity’s sake, this behavior is fine, but in a production thread pool implementation, you’d likely want to use std::thread::Builder and its spawn method that returns Result instead.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            if let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            } else {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ThreadPool};

    #[should_panic]
    #[test]
    fn it_should_panic_if_the_size_is_less_than_one() {
        ThreadPool::new(0);
    }

    #[test]
    fn it_should_return_thread_pool() {

    }
}