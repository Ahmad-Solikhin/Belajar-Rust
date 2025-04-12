use std::thread;
use std::time::Duration;

fn main() {
    let thread = thread::current();
    println!("{:?}", thread);
    println!("Hello, world!");
}

fn calculate() -> i32 {
    let thread = thread::current();
    println!("{:?}", thread);
    let mut counter = 0;

    for i in 1..=5 {
        println!("Counter : {}", i);
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::calculate;
    use std::cell::RefCell;
    use std::sync::{Arc, Barrier, Mutex, Once};
    use std::thread;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application finish");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });

        let result = handle.join();

        match result {
            Ok(val) => {
                println!("Result is {}", val);
            }
            Err(error) => {
                println!("Error is {:?}", error);
            }
        }

        println!("Application finish");
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Total 1 : {}, total 2 : {}", result1, result2);
        println!("Application finish");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => {
                println!("Result 1 is {}", counter)
            }
            Err(error) => {
                println!("Error 1 {:?}", error)
            }
        }

        match result2 {
            Ok(counter) => {
                println!("Result 1 is {}", counter)
            }
            Err(error) => {
                println!("Error 1 {:?}", error)
            }
        }

        println!("Application finish");
    }

    #[test]
    fn test_closure() {
        let thread = thread::current();
        println!("{:?}", thread);

        let name = String::from("Gayuh");

        //Melakukan move variable name ke dalam closure, dengan demikian variable name tidak akan bisa digunakan lagi
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello, my name is {}", name);
        };

        let handle = thread::spawn(closure);

        handle.join().unwrap();
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handle = factory
            .spawn(calculate)
            .expect("Failed to create a new thread");

        let result = handle.join().unwrap();
        println!("Result is {}", result);
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("Hello Ges".to_string());
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 1..6 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello Ges".to_string());
            }
            sender.send("Exit".to_string());
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();

                if message == "Exit".to_string() {
                    break;
                }
                println!("{}", message);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 1..6 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello Ges".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver {
                println!("{}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let sender2 = sender.clone();

        let handle1 = thread::spawn(move || {
            for i in 1..6 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello Ges".to_string());
            }
        });

        let handle3 = thread::spawn(move || {
            for i in 1..6 {
                thread::sleep(Duration::from_secs(1));
                sender2.send("Hello Bang".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver {
                println!("{}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
    }

    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000000 {
                    let mut data = counter_clone.lock().unwrap();
                    *data += 1;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let result = *counter.lock().unwrap();

        println!("Counter : {}", result);
    }

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
        pub static OTEHR_NAME: RefCell<String> = RefCell::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handle = thread::spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = "Gayuh".to_string();
            });

            NAME.with_borrow(|name| {
                println!("Name : {}", name);
            });
        });

        NAME.with_borrow(|name| {
            println!("Main name : {}", name);
        });

        handle.join().unwrap();

        NAME.with_borrow(|name| {
            println!("Main name : {}", name);
        });
    }

    #[test]
    fn test_thread_panic() {
        let handle = thread::spawn(|| {
            panic!("Haiyaaa error looo");
        });

        let result = handle.join();
        match result {
            Ok(_) => {
                println!("Success")
            }
            Err(_) => {
                println!("Thread panicked")
            }
        }

        println!("Test done");
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handles = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                println!("Join Game-{}", i + 1);
                barrier_clone.wait();
                println!("Gamer-{} Start", i + 1);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                TOTAL_COUNTER += 1;
            });

            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || {
                let total = get_total();
                println!("Total is : {}", total);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    async fn get_async_data() -> String {
        println!("Accessed async function");
        thread::sleep(Duration::from_secs(2));
        String::from("Return value async")
    }

    #[tokio::test]
    async fn test_async_future() {
        let result = get_async_data();
        let data = result.await; // Ini akan menunggu sampai dapet datanya
        println!("Data is : {}", data);
        println!("Haiyaa Looo");
    }

    async fn get_database_data(wait: u64) -> String {
        let mut actual_wait = wait;

        if wait == 8 {
            actual_wait = 1;
        }

        println!("{:?} : get database-{} data", thread::current(), wait);
        tokio::time::sleep(Duration::from_secs(actual_wait)).await;
        println!("{:?} : hello from database-{}", thread::current(), wait);
        format!("Return data from wait : {}", wait)
    }

    #[tokio::test]
    async fn test_concurrency() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = tokio::spawn(get_database_data(i));
            handles.push(handle);
        }

        let mut counter = 0;

        for handle in handles {
            if counter == 8 {
                counter += 1;
                continue;
            }

            let data = handle.await.unwrap();
            println!("response : {}", data);
            counter += 1;
        }
    }

    async fn run_concurrent(runtime: Arc<Runtime>) {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = runtime.spawn(get_database_data(i));
            handles.push(handle);
        }

        for handle in handles {
            let data = handle.await.unwrap();
            println!("response : {}", data);
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_time()
                .build()
                .unwrap()
        );

        runtime.block_on(run_concurrent(Arc::clone(&runtime)));
    }
}
