pub mod mt;
pub mod publisher;
pub mod pusher;
use std::thread;
use std::time::Duration;
use zmq;

pub fn NewServer() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:2222").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        responder.send(&msg.as_str().unwrap(), 0).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
