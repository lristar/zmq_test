use rand::distributions::{Distribution, Uniform};
use std::env;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use zmq;

pub fn new_publisher_path() {
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
    if let Err(e)=publisher.set_connect_timeout(10){
        eprintln!("e = {:?}", e);
        return;
    }
    if let Err(e) = publisher.set_heartbeat_timeout(10){
        eprintln!("e = {:?}", e);
        return;
    }
    // Ensure subscriber connection has time to complete
    sleep(Duration::from_millis(1000));

    // Send out all 1,000 topic messages
    // for topic_nbr in 0..1000 {
    // let mut rng = rand::thread_rng();
    // let topic_range = Uniform::new_inclusive(start, end);
    // publisher
    //     .send(&format!("{:03}",15), zmq::SNDMORE)
    //     .unwrap();
    // publisher.send("Save Roger", 0).unwrap();
    // publisher
    //     .send(&format!("{:03}",16), zmq::SNDMORE)
    //     .unwrap();
    // publisher.send("Save Roger", 0).unwrap();
    // }
    // Send one random update per second
    eprintln!("send ");
    // let topic_range = Uniform::new(start, end);
    // let _pub = publisher;
    // let ping = thread::spawn(move || {
    //     loop {
    //         if let Err(e) = _pub.send("ping", 0) {
    //             break;
    //         }
    //         sleep(Duration::from_millis(3000));
    //     }
    // });

    // ping.join().unwrap();
    loop {
       let num = publisher.get_heartbeat_timeout();
       match num {
           Ok(v) => {eprintln!("v = {:?}", v);},
           Err(e) => {eprintln!("e = {:?}", e);},
       }
        sleep(Duration::from_millis(1000));
        // let a = "hi";
        // let b = "halo";
        // publisher.send(&a, zmq::SNDMORE).unwrap();
        // publisher.send("Off with his head!", 0).unwrap();
        // sleep(Duration::from_millis(1000));
        // publisher.send(&b, zmq::SNDMORE).unwrap();
        // publisher.send("Off with his head!", 0).unwrap();
    }
}

pub fn new_publisher_psenv() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind("tcp://*:4102")
        .expect("failed binding publisher");

    loop {
        sleep(Duration::from_millis(1000));
        publisher.send("We don't want to see this", 0).unwrap();
        publisher.send("We would like to see this", 0).unwrap();
        thread::sleep(Duration::from_millis(1));
    }
}
