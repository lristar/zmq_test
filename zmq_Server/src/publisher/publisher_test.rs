use rand::distributions::{Distribution, Uniform};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use time;

use crate::publisher::models::Message;
use crate::publisher::publisher::{IZmqEngine, ZmqEngine};

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
    if let Err(e) = publisher.set_connect_timeout(10) {
        eprintln!("e = {:?}", e);
        return;
    }
    if let Err(e) = publisher.set_heartbeat_timeout(10) {
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
    let pub_clone = Arc::new(Mutex::new(&publisher));
    let pub_thread = Arc::clone(&pub_clone);
    // let ping = thread::spawn(move || loop {
    //     let mut p = pub_thread.lock().unwrap();
    //     p.send("ping", 0).expect("ping success");
    //     // pub_thread.send("ping", 0).expect("ping success");
    //     sleep(Duration::from_millis(3000));
    // });
    // ping.join().unwrap();
    loop {
        sleep(Duration::from_millis(1000));
        let a = "hi";
        let b = "halo";
        let mut p = pub_thread.lock().unwrap();
        p.send(&a, zmq::SNDMORE).unwrap();
        p.send("Off with his head!", 0).unwrap();
        sleep(Duration::from_millis(1000));
        p.send(&b, zmq::SNDMORE).unwrap();
        p.send("Off with his head!", 0).unwrap();
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

pub fn test_Publisher() {
    let mut zmq = ZmqEngine::new("tcp://*:4102", "tcp://*:2222", 10, 10).unwrap();
    eprintln!(" init success",);
    // let ping = thread::spawn(move || loop {
    //     mq1.check_ping().unwrap();
    //     // pub_thread.send("ping", 0).expect("ping success");
    //     sleep(Duration::from_millis(25000));
    // });
    zmq.start();

    let s = thread::spawn(move || loop {
        sleep(Duration::from_millis(10000));
        let m1 = Message {
            topic: "hi".to_string(),
            content: "We would like to see this".to_string(),
            time_sec: time::get_time().sec.to_string(),
        };
        zmq.pubish(m1).unwrap();
    });

    //    if let Some(handler) = &mq.threads {
    //         handler.join().unwrap();
    //    }
    // let p = thread::spawn(move || loop {
    //     let m = mq.resp().unwrap();
    //     mq.pubish(m.as_str(), "ping").unwrap();
    // });
    // p.join().unwrap();
    s.join().unwrap();
}
