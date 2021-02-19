use std::thread;
use std::time::Duration;
use std::time::SystemTime;

fn sing() {
    println!(" La la la ");
}

fn main() {
    let t1 = thread::spawn(|| {
        for _ in 0..5 {
            print!("{:?}",SystemTime::now());
            // wait 0.1 second
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    let t2 = thread::spawn(|| {
        for _ in 0..5 {
            sing();
            // wait 0.1 second
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // start s1 thread
    t1.join().unwrap();
    // start s2 thread
    t2.join().unwrap();
}
