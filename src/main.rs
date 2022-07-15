use std::time::Duration;
use std::sync::{Arc,Mutex,RwLock};
use std::convert::From;
use isis_eps_api_v2::*; // make dependency on git 
use cubeos_error::{Error,Result};
use cubeos_service::*;
use crate::eps;
use cubeos_service::{Config,Service};

// Make this subsystem.rs -------------------------------------------------
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

// Make this service.rs (macro) -------------------------------------------------
service_macro!{
    query: Ping => fn ping(&self) -> Result<()>;
    query: SystemReset => fn sys_reset(&self, typ: STID) -> Result<()>; in: STID; out: GqlExampleOutput; // define in graphql.rs
}

// Make this graphql.rs-------------------------------------------------
pub struct GqlInput {
    gql_in_no: i32,
}
#[derive(Serialize,Deserialize)]
pub struct GqlOutput {
    gql_out_no: Vec<f64>,
}

// Translate from gql to the defined inputs
// Do this for only the extra inputs 


// Keep this as main.rs-------------------------------------------------
fn main() -> epsResult<()> {
    
    // 
    let service_config = Config::new("isis_eps_service_v2")
    .map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
    })
    .unwrap();

    #[cfg(not(feature = "ground"))]
    let i2c_bus = service_config
    .get("i2c_bus")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();
    #[cfg(not(feature = "ground"))]
    let i2c_bus = i2c_bus.as_str().unwrap().to_string();


}
