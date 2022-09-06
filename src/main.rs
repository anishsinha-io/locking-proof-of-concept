use rand::Rng;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn insert_thread(map: Arc<RwLock<HashMap<i32, i32>>>) {
    loop {
        let k = rand::thread_rng().gen_range(0..10);
        let v = rand::thread_rng().gen_range(0..10);
        // lock the rwlock in write mode and if the result is okay, perform an insert operation
        if let Ok(mut write_guard) = map.write() {
            write_guard.insert(k, v);
            println!("CRITICAL SECTION (INSERT)-------------------------------------------------");
            println!("{:#?}", write_guard);
            println!(
                "inserted {}->{} into hashmap | hashmap length: {}",
                k,
                v,
                write_guard.len()
            );
            println!(
                "END CRITICAL SECTION (INSERT)-------------------------------------------------"
            );
        }
        // the write lock is released at the end of the if block above, because it fell out of scope. no need to manually release the lock
        thread::sleep(Duration::from_secs(1));
    }
}

fn remove_thread(map: Arc<RwLock<HashMap<i32, i32>>>) {
    loop {
        let k = rand::thread_rng().gen_range(0..10);
        // lock the rwlock in write mode and if the result is okay, perform a remove operation
        if let Ok(mut write_guard) = map.write() {
            write_guard.remove(&k);
            println!("CRITICAL SECTION (REMOVE)-------------------------------------------------");
            println!("{:#?}", write_guard);
            println!(
                "removed key at {} if it existed | hashmap length: {}",
                k,
                write_guard.len()
            );
            println!(
                "END CRITICAL SECTION (REMOVE)-------------------------------------------------"
            );
        }
        // the write lock is released at the end of the if block above, because it fell out of scope. no need to manually release the lock
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_thread(map: Arc<RwLock<HashMap<i32, i32>>>) -> i32 {
    loop {
        let k = rand::thread_rng().gen_range(0..10);
        // lock the rwlock in read mode and if the operation is okay, perform a get operation
        if let Ok(read_guard) = map.read() {
            let x = match read_guard.get(&k) {
                Some(val) => *val,
                None => -1,
            };
            println!("SHARED SECTION (GET)-------------------------------------------------");
            println!("{:#?}", read_guard);
            println!(
                "value at {} is {} | hashmap length: {}",
                k,
                x,
                read_guard.len()
            );
            println!("END SHARED SECTION (GET)-------------------------------------------------");
        }
        // the read lock is released at the end of the if block above, because it fell out of scope. no need to manually release the lock
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    // create thread safe map (protected by a RwLock)
    let thread_safe_map: Arc<RwLock<HashMap<i32, i32>>> = Arc::new(RwLock::new(HashMap::new()));
    // create a thread pool
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    // create clones of the map to pass to threads (underneath, they all point to the same map)
    let thread_one_copy = thread_safe_map.clone();
    let thread_two_copy = thread_safe_map.clone();
    let thread_three_copy = thread_safe_map.clone();
    let thread_four_copy = thread_safe_map.clone();
    let thread_five_copy = thread_safe_map.clone();
    let thread_six_copy = thread_safe_map.clone();
    // spawn six threads; two writers and four readers. the readers may overlap but the writers will block other operations until
    // they release their locks
    pool.spawn(move || insert_thread(thread_one_copy));
    pool.spawn(move || remove_thread(thread_two_copy));
    pool.spawn(move || {
        get_thread(thread_three_copy);
    });
    pool.spawn(move || {
        get_thread(thread_four_copy);
    });
    pool.spawn(move || {
        get_thread(thread_five_copy);
    });
    pool.spawn(move || {
        get_thread(thread_six_copy);
    });
    // just a little hack to make main run forever so that the threads aren't killed when main exits (since it won't exit)
    loop {}
}
