use std::env;

use rand::distributions::{Distribution, Uniform};

pub fn new_subscriber_path(topic: i32) {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();

    let args: Vec<_> = env::args().collect();
    let address = if args.len() == 2 {
        args[1].as_str()
    } else {
        "tcp://localhost:5556"
    };
    if let Err(e) = subscriber.set_reconnect_ivl_max(10) {
        eprintln!("e = {:?}", e);
    }

    subscriber
        .connect(&address)
        .expect("could not connect to publisher");

    eprintln!(" connect success ");

    // let mut rng = rand::thread_rng();
    // let topic_range = Uniform::new(15, 16);
    let subscription = format!("{:03}", topic).into_bytes();
    subscriber.set_subscribe(&subscription).unwrap();
    let event =  subscriber.get_events();
    match event {
        Ok(v) => {
        eprintln!("event = {:?}", v);
        },
        Err(e) => {eprintln!("e = {:?}", e);},
    }

    loop {
        let topic = subscriber.recv_msg(0).unwrap();
        let data = subscriber.recv_msg(0).unwrap();
        assert_eq!(&topic[..], &subscription[..]);
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}

pub fn new_subscriber_psenv() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://127.0.0.1:4102")
        .expect("failed connecting subscriber");
    let subStr = "15";
    // let sub = subStr.as_bytes();
    // eprintln!("&sub = {:?}", &sub);
    subscriber.set_subscribe(b"").expect("failed subscribing");
    subscriber.set_connect_timeout(30).expect("set timeout");
    loop {
        // let envelope = subscriber
        //     .recv_string(0)
        //     .expect("failed receiving envelope")
        //     .unwrap();
        let message = subscriber.recv_msg(0).expect("failed receiving message");
        println!(" {:#?}", message.as_str());
    }
}
