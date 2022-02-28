use std::{thread, time::Duration};

use zmq_Server;

fn main() {
    // zmq_Server::publisher::publisher::new_publisher_path();
    // zmq_Server::publisher::sync_publisher::new_sync_pub();
    zmq_Server::publisher::publisher_test::test_Publisher()
    // let a = test_thread();
    // let b = test_thread();
    // a.join().unwrap();
    // b.join().unwrap();
}

fn test_thread() -> thread::JoinHandle<()> {
    let f = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(5000));
        eprintln!(" hello world");
    });
    f
}
