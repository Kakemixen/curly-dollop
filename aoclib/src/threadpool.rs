use std::thread;
use std::sync::{
    mpsc, Arc, Mutex,
};


pub fn hello()
    -> ()
{
    println!("threadpool hello");
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker
{
    id: usize,
    //should join somewhere
    thread: thread::JoinHandle<()>,
}

impl Worker
{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)
        -> Worker
    {
        // TODO use reciever in thread
        let thread = thread::spawn(move || loop { 
            let job = reciever.lock().expect("poisoned reciever")
                .recv().expect("no dispatcher");
            job();
        });
        Worker { id, thread }
    }
}


pub struct ThreadPool
{
    workers: Vec<Worker>,
    dispatcher: mpsc::Sender<Job>,
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

    pub fn execute<F,T,R>(&self, f: F, input: T)
        -> mpsc::Receiver<R>
    where
        F: FnOnce(Box<T>) -> R + Send + 'static,
        T: std::marker::Send + 'static,
        R: std::marker::Send + 'static,
    {
        let input_box = Box::new(input);
        let job = Box::new(f);
        let (tx, rx) = mpsc::channel();

        if self.workers.len() == 0 {
            let out = job(input_box);
            tx.send(out).expect("no reciever for pool job!");
            return rx;
        }

        self.dispatcher.send(Box::new( move || { 
            let out = job(input_box);
            tx.send(out).expect("no reciever for pool job!");
        })).unwrap();

        //tx.send(job(input)).unwrap();
        rx
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use std::time;

    #[test]
    /// test return value of unthreaded pool is correct
    fn test_0x001()
    {
        let pool = ThreadPool::new(0);
        let func = | _ | {
            thread::sleep(time::Duration::from_millis(10));
            5
        };
        let now = time::Instant::now();

        let ret = match pool.execute(func,()).recv() {
            Ok(val) => val,
            Err(e) => { println!("recv error: {:?}", e); 0 },
        };
        assert!(now.elapsed() >= time::Duration::from_millis(10));
        assert_eq!(ret, 5);
    }

    #[test]
    /// test return value of threaded pool is correct
    fn test_0x002()
    {
        let pool = ThreadPool::new(2);
        let func = | _ | {
            thread::sleep(time::Duration::from_millis(10));
            5
        };
        let now = time::Instant::now();

        let reciever = pool.execute(func, ());
        assert!(now.elapsed() <= time::Duration::from_millis(10));
        assert_eq!(reciever.recv().unwrap(), 5);
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }

    #[test]
    /// test with capture
    fn test_0x003()
    {
        let pool = ThreadPool::new(2);
        let a = 2;
        let b = 3;
        let func = move | _ | {
            thread::sleep(time::Duration::from_millis(10));
            a + b
        };
        let now = time::Instant::now();

        let reciever = pool.execute(func, ());
        assert!(now.elapsed() <= time::Duration::from_millis(10));
        assert_eq!(reciever.recv().unwrap(), 5);
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }

    #[test]
    /// test with input
    fn test_0x004()
    {
        struct InputStruct { a: i32, b: i32 }

        let pool = ThreadPool::new(2);
        let input = InputStruct { a: 2, b: 3};
        let func = | input: Box<InputStruct> | {
            thread::sleep(time::Duration::from_millis(10));
            input.a + input.b
        };
        let now = time::Instant::now();

        let reciever = pool.execute(func, input);
        assert!(now.elapsed() <= time::Duration::from_millis(10));
        assert_eq!(reciever.recv().unwrap(), 5);
        assert!(now.elapsed() >= time::Duration::from_millis(10));
    }
}
