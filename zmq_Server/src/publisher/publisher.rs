use rand::distributions::{Distribution, Uniform};
use std::env;
use std::io::Error;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use zmq;

pub trait IZmqEngine {
    fn prepare(
        &mut self,
        pub_address: &str,
        resp_address: &str,
        connect_timeout: i32,
        heartbeat_timeout: i32,
    );
    fn start(&self);
    fn run(&self);
    fn stop(&self);
    fn pubish(&self, data: &str, topic: &str) -> result::Result<(), zmq::Error>;
    fn poll(&self);
    fn resp(&self);
    fn check_ping(&self) -> result::Result<(), zmq::Error>;
    fn check_connect(&self);
}

pub struct ZmqEngine {
    context: zmq::Context,
    pub_socket: Mutex<zmq::Socket>,
    rep_socket: Mutex<zmq::Socket>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl ZmqEngine {
    pub fn new(
        pub_address: &str,
        resp_address: &str,
        connect_timeout: i32,
        heartbeat_timeout: i32,
    ) -> result::Result<ZmqEngine, zmq::Error> {
        let ctx = zmq::Context::new();
        let pst = ctx.socket(zmq::PUB)?;
        let rpst = ctx.socket(zmq::REP)?;
        rpst.set_connect_timeout(connect_timeout)?;
        rpst.set_heartbeat_timeout(heartbeat_timeout)?;
        pst.set_connect_timeout(connect_timeout)?;
        pst.set_heartbeat_timeout(heartbeat_timeout)?;
        pst.bind(pub_address)?;
        rpst.bind(resp_address)?;
        Ok(ZmqEngine {
            context: ctx,
            pub_socket: Mutex::new(pst),
            rep_socket: Mutex::new(rpst),
            threads: vec![],
        })
    }
}

impl IZmqEngine for ZmqEngine {
    fn prepare(
        &mut self,
        pub_address: &str,
        resp_address: &str,
        connect_timeout: i32,
        heartbeat_timeout: i32,
    ) {
        let pb = self.pub_socket.try_lock().unwrap();
        let rp = self.rep_socket.try_lock().unwrap();
        pb.set_connect_timeout(connect_timeout).unwrap();
        pb.set_heartbeat_timeout(heartbeat_timeout).unwrap();
        pb.bind(pub_address).unwrap();
        rp.set_connect_timeout(connect_timeout).unwrap();
        rp.set_heartbeat_timeout(heartbeat_timeout).unwrap();
        rp.bind(resp_address).unwrap();
    }

    fn start(&self) {
        // check server which can use
        
    }

    fn run(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn pubish(&self, data: &str, topic: &str) -> result::Result<(), zmq::Error> {
        let m = match self.pub_socket.lock() {
            Ok(m) => m,
            Err(_e) => return Err(zmq::Error::EACCES),
        };
        // 定位topic
        m.send(topic, zmq::SNDMORE)?;
        Ok(m.send(data, 0)?)
    }

    fn resp(&self) {
        todo!()
    }

    fn poll(&self) {
        todo!()
    }

    fn check_connect(&self) {
        todo!()
    }

    fn check_ping(&self) -> result::Result<(), zmq::Error> {
        Ok(self.pubish("ping", "ping")?)
    }
}
