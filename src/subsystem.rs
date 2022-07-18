// 
use std::time::Duration;
use std::sync::{Arc,Mutex,RwLock};
use std::convert::From;
use isis_eps_api_v2::*; // move to CubeOS git
use cubeos_error::{Error,Result};
use cubeos_service::*;
use cubeos_service::{Config,Service};
use juniper::*;
use serde::*;
use std::convert::TryFrom;



#[derive(Clone)]
pub struct Subsystem {
    eps: Arc<Mutex<EPS>>,
    pub last_cmd: Arc<RwLock<Vec<u8>>>,
    pub last_err: Arc<RwLock<Error>>, // Error from cubeos
}

impl Subsystem {
    pub fn new(i2c_path: String, i2c_addr: u16,) -> epsResult<Self> {
        Ok(Self {
            eps: Arc::new(Mutex::new(EPS::new(i2c_path, i2c_addr)?)),
            last_cmd: Arc::new(RwLock::new(Vec::new())),
            last_err: Arc::new(RwLock::new(Error::None)),
        })
    }

    // All isis functions here
    pub fn sys_reset(&self, typ: STID) -> epsResult<()> {
        Ok(self.eps.lock().unwrap().sys_reset(typ)?)
    }

}