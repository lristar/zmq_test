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
    fn start(&mut self);
    fn run(&self);
    fn join(&self) -> Option<thread::JoinHandle<()>>;
    fn pubish(&self, data: &str, topic: &str) -> result::Result<(), zmq::Error>;
    fn poll(&self);
    fn resp(&self) -> result::Result<String, zmq::Error>;
    fn check_ping(&self) -> result::Result<(), zmq::Error>;
    fn check_connect(&self);
}

pub struct ZmqEngine {
    context: zmq::Context,
    pub_socket: Arc<Mutex<zmq::Socket>>,
    rep_socket: Arc<Mutex<zmq::Socket>>,
    is_active: bool,
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
            pub_socket: Arc::new(Mutex::new(pst)),
            rep_socket: Arc::new(Mutex::new(rpst)),
            is_active: false,
        })
    }
}

impl IZmqEngine for ZmqEngine {
    fn start(&mut self) {
        // check server which can use
        self.is_active = true
    }

    fn run(&self) {
        todo!()
    }

    // 启动线程，放在程序最后面
    fn join(&self) -> Option<thread::JoinHandle<()>> {
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

    fn resp(&self) -> result::Result<String, zmq::Error> {
        let rp = match self.rep_socket.lock() {
            Ok(m) => m,
            Err(_) => return Err(zmq::Error::EACCES),
        };
        let mut msg = zmq::Message::new();
        rp.recv(&mut msg, 0)?;
        let m = msg.as_str().unwrap();
        rp.send(m, 0)?;
        Ok(m.to_string())
    }

    fn poll(&self) {
        todo!()
    }

    fn check_connect(&self) {
        let ac = Arc::clone(&self.rep_socket);
        let pb = Arc::clone(&self.pub_socket);
        let handler = thread::spawn(move || {
            let rp = match ac.try_lock() {
                Ok(m) => m,
                Err(e) => panic!("check connect gei lock failed"),
            };
            let mut msg = zmq::Message::new();
            loop {
                match rp.recv(&mut msg, 0) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("check_connect recv is error = {:?}", e);
                    }
                }
                let result = &match msg.as_str() {
                    Some(e) => e,
                    None => {
                        continue;
                    }
                };
                match rp.send(result, 0) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("check_connect send is error = {:?}", e);
                    }
                }
                println!("start trylock");
                // match pb.try_lock() {
                    
                //     Ok(p) => {
                //         p.send("ping", zmq::SNDMORE).unwrap_or_else(|err| {eprintln!("ping send err is = {:?}",err );});
                //         p.send(result, 0).unwrap_or_else(|err| {eprintln!("ping send err is = {:?}",err );});
                //     }
                //     Err(e) => eprintln!(" = "),
                // }
            }
        });
        
    }

    fn check_ping(&self) -> result::Result<(), zmq::Error> {
        Ok(self.pubish("ping", "ping")?)
    }
}
