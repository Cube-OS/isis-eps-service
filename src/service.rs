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



// Make this service.rs (macro) -------------------------------------------------
service_macro!{
    query: SystemReset => fn sys_reset(&self, typ: STID) -> Result<()>; in: STID; // define in graphql.rs
}