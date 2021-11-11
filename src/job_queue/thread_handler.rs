use super::Job;
use std::{
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const DEFAULT_JOB_CAPACITY: usize = 256;

lazy_static! {
    pub(crate) static ref THREAD_HANDLER: Mutex<ThreadHandler> = Mutex::new(ThreadHandler::new());
}

struct Thread {
    handle: Option<JoinHandle<()>>,
    sender: Sender<ThreadMessage>,
}

enum ThreadMessage {
    Shutdown,
}

pub struct ThreadHandler {
    active_jobs: usize,
    jobs: Vec<Job>,
    threads: Vec<Thread>,
}

impl ThreadHandler {
    pub fn new() -> Self {
        // Cap the CPUs to 1 less than the max
        let num_cpus = if let Some(num) = num_cpus::get().checked_sub(1) {
            num
        } else {
            0
        };

        let mut threads = vec![];

        // Spawn threads
        for _ in 0..num_cpus {
            let (sender, receiver) = channel();

            // Spawn the actual thread
            let handle = thread::spawn(move || {
                let original_backoff = Duration::from_micros(10);
                let max_backoff = Duration::from_millis(1);
                let mut backoff = Duration::from_micros(10);

                // Endlessly loop the thread
                loop {
                    // Check parent messages.
                    for msg in receiver.try_iter() {
                        match msg {
                            // Exit if it should shutdown
                            ThreadMessage::Shutdown => return,
                        }
                    }

                    // Attempt to get a job from the queue
                    let job = {
                        let mut handler = THREAD_HANDLER.lock().unwrap();
                        if handler.jobs.is_empty() == false {
                            THREAD_HANDLER.lock().unwrap().decrement_jobs();

                            Some(handler.jobs.remove(0))
                        } else {
                            None
                        }
                    };

                    // Reset backoff + execute job if it exists
                    if let Some(job) = job {
                        backoff = original_backoff;
                        job();
                    } else {
                        // Otherwise exponential backoff to prevent it from going bonkers
                        backoff *= 2;
                        if backoff > max_backoff {
                            backoff = max_backoff;
                        }
                    }
                }
            });

            // Save handle
            threads.push(Thread {
                sender,
                handle: Some(handle),
            });
        }

        Self {
            active_jobs: 0,
            jobs: Vec::with_capacity(DEFAULT_JOB_CAPACITY), // arbitrary
            threads,
        }
    }

    /// Decrements all active jobs
    fn decrement_jobs(&mut self) {
        if self.active_jobs > 0 {
            self.active_jobs -= 1;
        }
    }

    /// The number of jobs currently processing
    pub fn num_jobs(&self) -> usize {
        self.active_jobs
    }

    /// Queues the given method.
    #[allow(unreachable_code)]
    pub fn queue(&mut self, f: Job) {
        // If no threads, simply execute the function.
        if self.threads.is_empty() {
            f();
        } else {
            // Push thread onto queue
            self.jobs.push(f);
            self.active_jobs += 1;
        }
    }
}

impl Drop for ThreadHandler {
    fn drop(&mut self) {
        // Clean up by joining all threads.
        for thread in self.threads.iter_mut() {
            // SHut it down
            match thread.sender.send(ThreadMessage::Shutdown) {
                Ok(_) => {}
                Err(_) => {}
            }

            // Join the thread
            if let Some(handle) = thread.handle.take() {
                match handle.join() {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
