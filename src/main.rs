// This is the service module for the ISIS Compact EPS

pub mod service;
pub mod subsystem;
pub mod graphql;


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


}
