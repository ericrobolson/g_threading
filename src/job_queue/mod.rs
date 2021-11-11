#[cfg(feature = "threaded")]
mod thread_handler;

/// Structure containing job queue related functionality.
pub struct JobQueue {}
pub type Job = alloc::boxed::Box<dyn FnOnce() + Send + 'static>;

impl JobQueue {
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
