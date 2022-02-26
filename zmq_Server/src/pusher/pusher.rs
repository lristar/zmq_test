use std::thread;
use std::io;

pub fn NewPush(){
  let context = zmq::Context::new();

    //  Socket to send messages to
    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    loop {
        // Send results to sink
        sender.send("", 0).unwrap();
    }
}