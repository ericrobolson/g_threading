A simple crate that exposes some multithreading functionality.

`job_queue`, `lock` and `channel` are the primary modules and should contain most things necessary.

If used in a non-threaded environment, it will run everything synchronously.

Roadmap:
- [x] Add in multithreading of jobs
- [ ] TODO: add in no-std channel implementations?
- [ ] TODO: add in RW lock for thread queue?
- [ ] Idea: [Convert to lockless work stealing?](https://blog.molecular-matters.com/2015/08/24/job-system-2-0-lock-free-work-stealing-part-1-basics/)