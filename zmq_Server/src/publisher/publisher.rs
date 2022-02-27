use rand::distributions::{Distribution, Uniform};
use std::env;
use std::io::Error;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use zmq;

pub trait IPubServer {
    fn Pub(&self, data: &str, topic: &str) -> result::Result<(), zmq::Error>;
    fn Poll(&self);
    fn check_ping(&self) -> result::Result<(), zmq::Error>;
}

pub struct ZmqEngine {
    context: zmq::Context,
    // pub_socket: zmq::Socket,
    pub_socket: Mutex<zmq::Socket>,
}

impl ZmqEngine {
    pub fn new(
        pub_address: &str,
        connect_timeout: i32,
        heartbeat_timeout: i32,
    ) -> result::Result<ZmqEngine, zmq::Error> {
        let ctx = zmq::Context::new();
        let pst = ctx.socket(zmq::PUB)?;
        pst.set_connect_timeout(connect_timeout)?;
        pst.set_heartbeat_timeout(heartbeat_timeout)?;
        pst.bind(pub_address)?;
        Ok(ZmqEngine {
            context: ctx,
            pub_socket: Mutex::new(pst),
        })
    }
}


impl IPubServer for ZmqEngine {
    fn Pub(&self, data: &str, topic: &str) -> result::Result<(), zmq::Error> {
        let m = match self.pub_socket.lock() {
            Ok(m) => m,
            Err(e) => return Err(zmq::Error::EACCES),
        };
        // 定位topic
        m.send(topic, zmq::SNDMORE)?;
        Ok(m.send(data, 0)?)
    }

    fn Poll(&self) {
        todo!()
    }

    fn check_ping(&self) -> result::Result<(), zmq::Error> {
        Ok(self.Pub("ping", "ping")?)
    }
}
