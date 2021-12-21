use std::thread;
use std::sync::{
    mpsc, Arc, Mutex,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message
{
    NewJob(Job),
    Terminate,
}

struct Worker
{
    id: usize,
    //should join somewhere
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker
{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>)
        -> Worker
    {
        let thread = thread::Builder::new()
            .name(format!("threadpool worker {}", id))
            .spawn(move || loop {
                let message = reciever.lock().expect("poisoned reciever")
                    .recv().expect("no dispatcher");
                match message {
                    Message::NewJob(job) => job(),
                    Message::Terminate => break,
                }

        }).unwrap();
        Worker { id, thread: Some(thread) }
    }
}


pub struct ThreadPool
{
    workers: Vec<Worker>,
    dispatcher: mpsc::Sender<Message>,
}

impl ThreadPool
{
    pub fn new(size: usize)
        -> ThreadPool
    {
        let (dispatcher, reciever) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let reciever = Arc::new(Mutex::new(reciever));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            workers, dispatcher
        }
    }

    pub fn execute<F,>(&self, f: F )
        -> ()
    where
        F: FnOnce() -> () + Send + 'static,
    {
        if self.workers.len() == 0 {
            f();
            return;
        }

        self.dispatcher.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.dispatcher.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join()
                    .expect(&format!("can't join thread {}", worker.id));
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use std::time;

    #[test]
    /// test time of unthreaded pool is correct
    fn test_0x001()
    {
        let pool = ThreadPool::new(0);
        let func = | | {
            thread::sleep(time::Duration::from_millis(10));
        };
        let now = time::Instant::now();

        pool.execute(func);
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }

    #[test]
    /// test time of threaded pool is correct
    fn test_0x002()
    {
        let now = time::Instant::now();
        {
            let pool = ThreadPool::new(2);
            let func = | | {
                thread::sleep(time::Duration::from_millis(10));
            };

            pool.execute(func);
            assert!(now.elapsed() <= time::Duration::from_millis(10));
        }
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }

    #[test]
    /// test result with capture and channel
    fn test_0x003()
    {
        let pool = ThreadPool::new(2);
        let a = 2;
        let b = 3;
        let (transmitter, reciever) = mpsc::channel();
        let func = move | | {
            thread::sleep(time::Duration::from_millis(10));
            transmitter.send(a + b).unwrap();
        };
        let now = time::Instant::now();

        pool.execute(func);
        assert!(now.elapsed() <= time::Duration::from_millis(10));
        assert_eq!(reciever.recv().unwrap(), 5);
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }
}
