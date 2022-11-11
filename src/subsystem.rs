//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Contributed by: Xueliang Bai (xbai9225@gmail.com)
//
// In this file the subsystem that contains all the functions to interact with the API is defined
//
// Comments generated in parts with GPT-3 (see disclaimer in README)

// use std::convert::From;
use cubeos_error::{Error, Result};
use isis_eps_api::*;  
use cubeos_service::*;
// include output of macro in cubeos_service
use crate::service::*;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use juniper::*;
use serde::*;
// use std::convert::TryFrom;

const PIU_STID: u8 = 0x1A;

#[derive(Clone)]
pub struct Subsystem {
    eps: Arc<Mutex<EPS>>,
    pub last_cmd: Arc<RwLock<Vec<u8>>>,
    pub last_err: Arc<RwLock<Error>>, // Error from cubeos
}

impl Subsystem {
    /// Initialisation of the Subsystem
    ///
    /// # Arguments
    ///
    /// * `i2c_path` - A string that represents the path to the i2c device.
    /// * `i2c_addr` - A u16 that represents the address of the i2c device.
    /// * `udp_path` - A serial::PortSettings that represents service address on the ground
    /// * `udp_addr` - A serial::PortSettings that represents the service address (IP/Port) on the satellite.
    /// # Output
    ///
    /// *  EpsResult<Self>` - Returns `Self` or EpsError.
    /// 
    pub fn new(i2c_path: String, i2c_addr: u16, udp_path: String, udp_to: String) -> EpsResult<Self> {
        Ok(Self {
            eps: Arc::new(Mutex::new(EPS::new(i2c_path, i2c_addr, udp_path, udp_to)?)),
            last_cmd: Arc::new(RwLock::new(Vec::new())),
            last_err: Arc::new(RwLock::new(Error::None)),
        })
    }

    // All isis functions here
    pub fn eps_ping(&self) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().eps_ping(typ_id)?)
    }

    pub fn sys_reset(&self, ret_key: u8) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;        
        Ok(self.eps.lock().unwrap().sys_reset(typ_id, ret_key)?)
    }

    pub fn shutdown_all(&self) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().shutdown_all(typ_id)?)
    }

    pub fn watchdog_reset(&self) -> EpsResult<()> {        
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().watchdog_reset(typ_id)?)
    }

    pub fn set_group_outputs(&self, typ_group: BusGroup, eps_bitflag: u16) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().set_group_outputs(typ_id, typ_group, eps_bitflag)?)
    }

    pub fn set_single_output(&self, typ_channel: BusChannel, eps_ch_idx: u8) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().set_single_output(typ_id, typ_channel, eps_ch_idx)?)
    }

    pub fn mode_switch(&self, mode: ModeSwitch) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().mode_switch(typ_id, mode)?)    
    }

    pub fn system_status(&self) -> EpsResult<SystemStatus> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().system_status(typ_id)?)
    }

    pub fn overcurrent_state(&self) -> EpsResult<OverCurrentFaultState> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().overcurrent_state(typ_id)?)
    }

    pub fn abf_state(&self) -> EpsResult<ABFState> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().abf_state(typ_id)?)
    }

    pub fn pdu_hk(&self, mode: PDUHkSel) -> EpsResult<PDUHk> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().pdu_hk(typ_id, mode)?)
    }

    pub fn pbu_hk(&self, mode: PBUHkSel) -> EpsResult<PBUHk> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().pbu_hk(typ_id, mode)?)
    }

    pub fn pcu_hk(&self, mode: PCUHkSel) -> EpsResult<PCUHk> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid; 
        Ok(self.eps.lock().unwrap().pcu_hk(typ_id, mode)?)
    }

    pub fn piu_hk(&self, mode: PIUHkSel) -> EpsResult<PIUHk> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().piu_hk(typ_id, mode)?)
    }

    pub fn system_config_cmd(&self, mode: SysConfig1, para_id: u16) -> EpsResult<Vec<u8>> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;        
        Ok(self.eps.lock().unwrap().system_config_command(typ_id, mode, para_id)?)   
    }

    pub fn reset_all_conf(&self, mode: SysConfig2, config_key: u8) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().reset_all_conf(typ_id, mode, config_key)?)   
    }

    pub fn correct_time(&self, time_correction: i32) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().correct_time(typ_id, time_correction)?)   
    }

    pub fn reset_all_counters(&self, zero_key: u8) -> EpsResult<()> {
        // hard coded the typ_id to be PiuStid
        let typ_id = StID::PiuStid;
        Ok(self.eps.lock().unwrap().reset_all_counters(typ_id, zero_key)?)   
    }
}