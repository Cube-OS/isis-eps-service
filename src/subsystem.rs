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

use cubeos_service::Error;
use isis_eps_api::*;
use std::sync::{Arc, Mutex, RwLock};

// include output of macro in cubeos_service

#[derive(Clone)]
pub struct Subsystem {
    eps: Arc<Mutex<Eps>>,
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
    pub fn new(i2c_path: String, i2c_addr: u16) -> EpsResult<Self> {
        Ok(Self {
            eps: Arc::new(Mutex::new(Eps::new(i2c_path, i2c_addr)?)),
            last_cmd: Arc::new(RwLock::new(Vec::new())),
            last_err: Arc::new(RwLock::new(Error::None)),
        })
    }

    // All isis functions here
    pub fn eps_ping(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().eps_ping()?)
    }

    pub fn sys_reset(&self, _ret_key: u8) -> EpsResult<()> {
        let ret_key = 0xA6;
        Ok(self.eps.lock().unwrap().sys_reset(ret_key)?)
    }

    pub fn shutdown_all(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().shutdown_all()?)
    }

    pub fn watchdog_reset(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().watchdog_reset()?)
    }

    pub fn set_group_outputs(&self, typ_group: BusGroup, channels: Vec<u8>) -> EpsResult<()> {
        Ok(self
            .eps
            .lock()
            .unwrap()
            .set_group_outputs(typ_group, channels)?)
    }

    pub fn set_group_state(&self, typ_group: BusGroup, state: BusChannelState) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().set_group_state(typ_group, state)?)
    }

    pub fn set_single_output(&self, typ_channel: BusChannel, eps_ch_idx: u8) -> EpsResult<()> {
        Ok(self
            .eps
            .lock()
            .unwrap()
            .set_single_output(typ_channel, eps_ch_idx)?)
    }

    pub fn mode_switch(&self, mode: ModeSwitch) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().mode_switch(mode)?)
    }

    pub fn system_status(&self) -> EpsResult<SystemStatus> {
        Ok(self.eps.lock().unwrap().system_status()?)
    }

    pub fn overcurrent_state(&self) -> EpsResult<OverCurrentFaultState> {
        Ok(self.eps.lock().unwrap().overcurrent_state()?)
    }

    // pub fn abf_state(&self) -> EpsResult<ABFState> {
    //     Ok(self.eps.lock().unwrap().abf_state()?)
    // }

    pub fn pdu_hk(&self, mode: PDUHkSel) -> EpsResult<PDUHk> {
        Ok(self.eps.lock().unwrap().pdu_hk(mode)?)
    }

    pub fn pbu_hk(&self, mode: PBUHkSel) -> EpsResult<PBUHk> {
        Ok(self.eps.lock().unwrap().pbu_hk(mode)?)
    }

    pub fn pcu_hk(&self, mode: PCUHkSel) -> EpsResult<PCUHk> {
        Ok(self.eps.lock().unwrap().pcu_hk(mode)?)
    }

    pub fn piu_hk(&self, mode: PIUHkSel) -> EpsResult<PIUHk> {
        Ok(self.eps.lock().unwrap().piu_hk(mode)?)
    }

    // pub fn system_config_cmd(&self, mode: SysConfig1, para_id: u16) -> EpsResult<Vec<u8>> {
    //     Ok(self.eps.lock().unwrap().system_config_command(mode, para_id)?)
    // }

    pub fn correct_time(&self, time_correction: i32) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().correct_time(time_correction)?)
    }

    pub fn reset_all_counters(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().reset_all_counters()?)
    }

    pub fn get_config_para_write(&self, param: ConfigParamWrite) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().get_config_para_write(param)?)
    }
    pub fn get_config_para_read(&self, param: ConfigParamRead) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().get_config_para_read(param)?)
    }
    pub fn set_config_para_u32(&self, param: ConfigParamWriteU32, input: u32) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().set_config_para_u32(param, input)?)
    }
    pub fn set_config_para_u16(&self, param: ConfigParamWriteU16, input: u16) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().set_config_para_u16(param, input)?)
    }
    pub fn set_config_para_i16(&self, param: ConfigParamWriteI16, input: i16) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().set_config_para_i16(param, input)?)
    }
    pub fn set_config_para_u8(&self, param: ConfigParamWriteU8, input: u8) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().set_config_para_u8(param, input)?)
    }
    pub fn set_config_para_i8(&self, param: ConfigParamWriteI8, input: i8) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().set_config_para_i8(param, input)?)
    }
    pub fn reset_param(&self, param: ConfigParamWrite) -> EpsResult<Output> {
        Ok(self.eps.lock().unwrap().reset_param(param)?)
    }
    pub fn reset_all_conf(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().reset_all_conf()?)
    }
    pub fn load_config(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().load_config()?)
    }
    pub fn force_save_config(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().save_config_force()?)
    }
    pub fn save_config(&self) -> EpsResult<()> {
        Ok(self.eps.lock().unwrap().save_config()?)
    }
}
