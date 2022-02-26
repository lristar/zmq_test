use std::io;
use std::thread;

pub fn NewPull() {
    let context = zmq::Context::new();

    // socket to receive messages on
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());


    loop {
        let msg = receiver.recv_string(0).unwrap().unwrap();
        eprintln!("msg = {:?}", &msg);
    }
}
