#[cfg(feature = "threaded")]
mod thread_handler;

/// Structure containing job queue related functionality.
pub struct JobQueue {}
pub type Job = alloc::boxed::Box<dyn FnOnce() + Send + 'static>;

impl JobQueue {
    /// Blocks until all jobs are finished
    #[allow(unreachable_code)]
    pub fn block() {
        while Self::num_jobs() > 0 {
            // block...
        }
    }

    /// The number of jobs currently processing
    #[allow(unreachable_code)]
    pub fn num_jobs() -> usize {
        // Queue up job if threading is enabled
        #[cfg(feature = "threaded")]
        {
            return thread_handler::THREAD_HANDLER.lock().unwrap().num_jobs();
        }

        0
    }

    /// Queues up the given job for execution.
    #[allow(unreachable_code)]
    pub fn queue(f: Job) {
        // Queue up job if threading is enabled
        #[cfg(feature = "threaded")]
        {
            thread_handler::THREAD_HANDLER.lock().unwrap().queue(f);
            return;
        }

        // Otherwise execute the function since it's a single threaded environment.
        f();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn test_many_jobs() {
        lazy_static! {
            pub(crate) static ref I: Mutex<i32> = Mutex::new(0);
        }

        for _ in 0..10 {
            JobQueue::queue(Box::new(|| {
                let mut data = I.lock().unwrap();
                *data += 1;
            }));
        }

        while JobQueue::num_jobs() > 0 {}

        assert_eq!(0, JobQueue::num_jobs());
        assert_eq!(10, *I.lock().unwrap());
    }
}
