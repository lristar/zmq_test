use zmq;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use rand::distributions::{Distribution, Uniform};
use std::thread;

pub fn new_publisher_path(start: i32, end:i32){
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    let args: Vec<_> = env::args().collect();
    let address = if args.len() == 2 {
        args[1].as_str()
    } else {
        "tcp://*:4102"
    };
    publisher
        .bind(&address)
        .expect("could not bind publisher socket");

    // Ensure subscriber connection has time to complete
    sleep(Duration::from_millis(1000));

    // Send out all 1,000 topic messages
    // for topic_nbr in 0..1000 {
        let mut rng = rand::thread_rng();
        let topic_range = Uniform::new_inclusive(start, end);
        publisher
            .send(&format!("{:03}",15), zmq::SNDMORE)
            .unwrap();
        publisher.send("Save Roger", 0).unwrap();
        publisher
            .send(&format!("{:03}",16), zmq::SNDMORE)
            .unwrap();
        publisher.send("Save Roger", 0).unwrap();
    // }
    // Send one random update per second
    eprintln!("send ");
    let topic_range = Uniform::new(start, end);
    loop {
        sleep(Duration::from_millis(1000));
        publisher
            .send(
                &format!("{:03}", 15),
                zmq::SNDMORE,
            )
            .unwrap();
        publisher.send("Off with his head!", 0).unwrap();
        sleep(Duration::from_millis(1000));
        publisher
            .send(
                &format!("{:03}", 16),
                zmq::SNDMORE,
            )
            .unwrap();
        publisher.send("Off with his head!", 0).unwrap();
        }
}

pub fn new_publisher_psenv(){
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind("tcp://*:4102")
        .expect("failed binding publisher");

    loop {
        sleep(Duration::from_millis(1000));
        publisher
            .send("We don't want to see this", 0)
            .expect("failed sending first message");
        publisher
            .send("We would like to see this", 0)
            .expect("failed sending second message");
        thread::sleep(Duration::from_millis(1));
    }
}