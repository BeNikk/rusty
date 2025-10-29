use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    // main thread finishes before the spawned thread,so we,
    // do not necessarily see the counting from 1 to 10.
    // to handle this we use the join and unwrap
    handle.join().unwrap();
    // main thread waits for this handle.join.unwrap();
}
