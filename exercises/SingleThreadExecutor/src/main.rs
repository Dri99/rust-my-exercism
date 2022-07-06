use single_thread_executor::SingleThreadExecutor;
fn main() {
    let func = Box::new(|| {println!("Hello")});
    let func2= Box::new(|| {println!("world!")});
    let executor = SingleThreadExecutor::new();
    executor.submit(func).unwrap();
    executor.submit(func2).unwrap();
    executor.close();
    executor.join();
}
