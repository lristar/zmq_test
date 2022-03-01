use rand::distributions::{Distribution, Uniform};
use serde_json::Result;
use std::fmt::{self, write};
use std::io::Error;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::{env, error, io};
use zmq;

use super::models;

#[derive(Debug)]
pub enum ErrorType {
    ZmqError(String),
    IoError(String),
    JsonError(String),
}

// impl fmt::Display for ErrorType{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write(output, args)
// `     }
// }
pub trait IZmqEngine {
    fn start(&mut self);
    fn run(&self);
    fn join(&self) -> Option<thread::JoinHandle<()>>;
    fn pubish(&self, msg: models::Message) -> result::Result<(), ErrorType>;
    fn poll(&self);
    fn resp(&self) -> result::Result<String, ErrorType>;
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
    fn pubish(&self, msg: models::Message) -> result::Result<(), ErrorType> {
        let m = match self.pub_socket.lock() {
            Ok(m) => m,
            Err(_e) => {
                return Err(ErrorType::IoError(
                    "zmq publish get lock failed".to_string(),
                ))
            }
        };
        let msg_json = match serde_json::to_string(&msg) {
            Ok(v) => v,
            Err(e) => return Err(ErrorType::JsonError("json exchange false".to_string())),
        };
        // 定位topic
        if let Err(e) = m.send(msg.topic.as_str(), zmq::SNDMORE) {
            return Err(ErrorType::ZmqError("zmq publish topic failed".to_string()));
        }
        if let Err(e) = m.send(msg_json.as_str(), 0) {
            return Err(ErrorType::ZmqError("zmq publish data failed".to_string()));
        }
        let result = self.resp()?;
        if result == msg.time_sec.to_string() {
            return Ok(());
        }
        return Err(ErrorType::IoError("sync server failed".to_string()));
    }

    fn resp(&self) -> result::Result<String, ErrorType> {
        let rp = match self.rep_socket.lock() {
            Ok(m) => m,
            Err(_) => {
                return Err(ErrorType::IoError(
                    "zmq publish get lock failed".to_string(),
                ))
            }
        };
        let mut msg = zmq::Message::new();
        if let Err(e) = rp.recv(&mut msg, 0) {
            return Err(ErrorType::ZmqError("zmq recv failed".to_string()));
        }
        let m = msg.as_str().unwrap();
        if let Err(e) = rp.send(m, 0) {
            return Err(ErrorType::ZmqError("zmq resp send failed".to_string()));
        }
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

    // fn check_ping(&self) -> result::Result<(), ErrorType> {
    //     Ok(self.pubish("ping", "ping")?)
    // }
}
