
fn main() {
    let context = zmq::Context::new();

    // socket to receive messages on
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    //  Socket to send messages to
    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();

        // Show progress
        print!(".");
        let _ = io::stdout().flush();

        // Do the work
        thread::sleep(Duration::from_millis(atoi(&string)));

        // Send results to sink
        sender.send("", 0).unwrap();
    }
}
