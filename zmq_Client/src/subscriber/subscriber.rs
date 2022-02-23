use std::env;

use rand::distributions::{Distribution, Uniform};

pub fn new_subscriber_path(topic:i32){
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();

    let args: Vec<_> = env::args().collect();
    let address = if args.len() == 2 {
        args[1].as_str()
    } else {
        "tcp://localhost:5556"
    };
    subscriber
        .connect(&address)
        .expect("could not connect to publisher");
    
    eprintln!(" connect success ");

    // let mut rng = rand::thread_rng();
    // let topic_range = Uniform::new(15, 16);
    let subscription = format!("{:03}", topic).into_bytes();
    subscriber.set_subscribe(&subscription).unwrap();

    loop {
        let topic = subscriber.recv_msg(0).unwrap();
        let data = subscriber.recv_msg(0).unwrap();
        assert_eq!(&topic[..], &subscription[..]);
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}

pub fn new_subscriber_psenv(){
    let version = zmq::version();
    eprintln!("version = {:?}", version);
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5563")
        .expect("failed connecting subscriber");
    subscriber.set_subscribe(b"A").expect("failed subscribing");

    loop {
        let envelope = subscriber
            .recv_string(0)
            .expect("failed receiving envelope")
            .unwrap();
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving message")
            .unwrap();
        println!("[{}] {}", envelope, message);
    }
}
