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

    #[cfg(not(feature = "ground"))]
    let i2c_bus = i2c_bus.as_str().unwrap().to_string();

    // Alternatively the I2C address can be hardcoded here
    #[cfg(not(feature = "ground"))]
    let i2c_addr = service_config
    .get("i2c_addr")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();
    #[cfg(not(feature = "ground"))]
    let i2c_addr = i2c_addr.as_str().unwrap();
    #[cfg(not(feature = "ground"))]
    let i2c_addr: u16 = if i2c_addr.starts_with("0x") {
        u16::from_str_radix(&i2c_addr[2..], 16).unwrap()
    } else {
        u16::from_str_radix(i2c_addr, 16).unwrap()
    };

    #[cfg(not(feature = "ground"))]
    let subsystem: Box<Subsystem> = Box::new(
        match Subsystem::new(i2c_path,i2c_addr)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            }) {
                Ok(b) => b,
                Err(e) => {
                    info!("Failed to create subsystem");
                    panic!("Subsystem creation failed: {:?}", e);
                }
            },
    );

    // Only needed for the ground feature
    #[cfg(feature = "ground")]
    let socket = service_config
    .get("udp_socket")
    .ok_or_else(|| {
        error!("Failed to load 'udp-socket' config value");
        format_err!("Failed to load 'udp-socket' config value");
    })
    .unwrap();

    #[cfg(feature = "ground")]
    let target = service_config
    .get("target")
    .ok_or_else(|| {
        error!("Failed to load 'target' config value");
        format_err!("Failed to load 'target' config value");
    })
    .unwrap();

    #[cfg(feature = "ground")]
    // Start debug service
    Service::new(
        service_config,
        QueryRoot,
        MutationRoot,
        socket.as_str().unwrap().to_string(),
        target.as_str().unwrap().to_string(),
    ).start();

    #[cfg(feature = "graphql")]
    // Start up graphql server
    Service::new(
        service_config,
        subsystem,
        QueryRoot,
        MutationRoot,
    )
    .start();

    #[cfg(not(any(feature = "ground",feature = "graphql")))]
    //Start up UDP server
    Service::new(
        service_config,
        subsystem,
        Some(Arc::new(udp_handler)),
    )
    .start();

    #[cfg(debug)]
    println!("{:?}", service_config);

    #[cfg(debug)]
    debug();

    Ok(())
}
