use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let nums = Arc::new(Mutex::new(vec![]));
    let mut thread_handers = vec![];
    for n in 0..5 {
        let ns = nums.clone();
        let c = thread::spawn(move || {
            let mut lock = ns.lock().unwrap();
            lock.push(n);
        }).join().unwrap();
        thread_handers.push(c);
    }

    // for c in thread_handers {
    //     c.join().unwrap();
    // }
    let mut value = nums.lock().unwrap();
    let vec = value.deref_mut();
    
    println!("{vec:?}");
}
