// This is the service module for the ISIS Compact EPS

pub mod service;
pub mod subsystem;

// include API
use isis_eps_api::*;  

use cubeos_service::*;

use crate::subsystem::Subsystem;
use crate::service::*;

use std::sync::Arc;






use log::{info,error};
use failure::format_err;

// -------------------------main.rs---------------------------------
fn main() -> EpsResult<()> {
    let service_config = Config::new("isis-eps-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    // Define i2c bus 
    #[cfg(not(feature = "terminal"))]
    let i2c_bus = service_config
        .get("i2c_bus")
        .ok_or_else(|| {
            error!("Failed to load 'i2c_bus' config value");
            format_err!("Failed to load 'i2c_bus' config value");
        })
        .unwrap();
    #[cfg(not(feature = "terminal"))]
    let i2c_bus = i2c_bus.as_str().unwrap().to_string();

    // Define the i2c_bus address in hex. Works with or without 0x. 
    #[cfg(not(feature = "terminal"))]
    let i2c_addr = service_config
    .get("i2c_addr")
    .ok_or_else(|| {
        error!("Failed to load 'i2c_addr' config value");
        format_err!("Failed to load 'i2c_addr' config value");
    })
    .unwrap();
    #[cfg(not(feature = "terminal"))]
    let i2c_addr = i2c_addr.as_str().unwrap();
    #[cfg(not(feature = "terminal"))]
    let i2c_addr: u16 = if i2c_addr.starts_with("0x") {
        u16::from_str_radix(&i2c_addr[2..], 16).unwrap()
    } else {
        u16::from_str_radix(i2c_addr, 16).unwrap()
    };
    
    // Only needed for the ground feature
    #[cfg(feature = "terminal")]
    let socket = service_config
        .get("udp_socket")
        .ok_or_else(|| {
            error!("Failed to load 'udp-socket' config value");
            format_err!("Failed to load 'udp-socket' config value");
        })
        .unwrap();

    #[cfg(feature = "terminal")]
    let target = service_config
        .get("target")
        .ok_or_else(|| {
            error!("Failed to load 'target' config value");
            format_err!("Failed to load 'target' config value");
        })
        .unwrap();    

    #[cfg(not(feature = "terminal"))]
    let subsystem: Box<Subsystem> = Box::new(
        match Subsystem::new(i2c_bus, i2c_addr)
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

    #[cfg(feature = "terminal")]
    // Start gs-auto service
    Service::new(
        service_config,
        socket.as_str().unwrap().to_string(),
        target.as_str().unwrap().to_string(),
        std::env::args().collect::<Vec<String>>(),
        Arc::new(input),
        Arc::new(output),
    ).start();

    //Start up UDP server for the Satellite
     #[cfg(not(any(feature = "terminal", feature = "ground", feature = "gs-auto", feature = "gs-schedule")))]
    Service::new(
        service_config,
        subsystem,
        Some(Arc::new(udp_handler)),
    )
    .start();

    Ok(())
}
