//
// Copyright (C) 2019 Kubos Corporation
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

// Implementation of GraphQL structs to use in the Cube-OS framework. 
// Only used in the "Ground" and "GraphQL" feature
// 
// GraphQL only allows four different types:
// i32,f64,String and bool
// 
// Therefore it is often necessary and desired (to reduce a data overhead over UDP)
// to translate from those types to types in the corresponding API or input struct 

use std::time::Duration;
use std::sync::{Arc,Mutex,RwLock};
use std::convert::From;
use isis_eps_api::*; // move to CubeOS git
use cubeos_error::{Error as CubeOSError, Result as CubeOSResult};
use cubeos_service::*;
use cubeos_service::{Config,Service};
use juniper::*;
use serde::*;
use std::convert::TryFrom;

// Output GQL Structs for conversion，will be re-implemented in CubeOS in future
// #[derive(Serialize, Deserialize)]
// pub struct GqlExampleOutput {
//     gql_out_no: Vec<f64>,
//     gql_out_str: String,
//     gql_out_bool: Vec<bool>,
// }
// // Translation from Output to GraphQLOutput
// impl From<ExampleOutput> for GqlExampleOutput {
//     fn from(o: ExampleOutput) -> GqlExampleOutput {
//         GqlExampleOutput {
//             gql_out_no: {
//                 let mut vec: Vec<f64> = Vec::new();
//                 for i in 0..o.out_no.len() {
//                     vec.push(o.out_no[i] as f64 * 1.4);
//                 }
//                 vec
//             },
//             gql_out_str: o.out_str,
//             gql_out_bool: o.out_bool,
//         }
//     }
// }

//SystemStatus
#[derive(Serialize, Deserialize)]
pub struct GqlSystemStatus {
    gql_mode: f64,
    gql_conf: f64,
    gql_reset_cause: f64,
    gql_uptime: f64,
    gql_error: f64,
    gql_rc_cnt_pwron: f64,
    gql_rc_cnt_wdg: f64,
    gql_rc_cnt_cmd: f64,
    gql_rc_cnt_mcu: f64,
    gql_rc_cnt_lowpwr: f64,
    gql_prevcmd_elapsed: f64,
    gql_unix_time: f64,
    gql_unix_year: f64,
    gql_unix_month: f64,
    gql_unix_day: f64,
    gql_unix_hour: f64,
    gql_unix_minute: f64,
    gql_unix_second: f64,
}

impl GqlSystemStatus {
    pub fn from_system_status(system_status: &SystemStatus) -> Self {
        GqlSystemStatus {
            gql_mode: system_status.mode as f64,
            gql_conf: system_status.conf as f64,
            gql_reset_cause: system_status.reset_cause as f64,
            gql_uptime: system_status.uptime as f64,
            gql_error: system_status.error as f64,
            gql_rc_cnt_pwron: system_status.rc_cnt_pwron as f64,
            gql_rc_cnt_wdg: system_status.rc_cnt_wdg as f64,
            gql_rc_cnt_cmd: system_status.rc_cnt_cmd as f64,
            gql_rc_cnt_mcu: system_status.rc_cnt_mcu as f64,
            gql_rc_cnt_lowpwr: system_status.rc_cnt_lowpwr as f64,
            gql_prevcmd_elapsed: system_status.prevcmd_elapsed as f64,
            gql_unix_time: system_status.unix_time as f64,
            gql_unix_year: system_status.unix_year as f64,
            gql_unix_month: system_status.unix_month as f64,
            gql_unix_day: system_status.unix_day as f64,
            gql_unix_hour: system_status.unix_hour as f64,
            gql_unix_minute: system_status.unix_minute as f64,
            gql_unix_second: system_status.unix_second as f64,
        }
    }
}

//OverCurrentFaultState
#[derive(Serialize, Deserialize)]
pub struct GqlOverCurrentFaultState {
    // One reseved byte. Starting from the 6th byte
    // Length of useful data for ICEPSv2 (17 channels), 50bytes
    gql_stat_ch_on: i32,
    gql_stat_ch_ext_on: i32,
    gql_stat_ch_ocf: i32,
    gql_stat_ch_ext_ocf: i32,
    gql_ocf_cnt_ch00: i32,
    gql_ocf_cnt_ch01: i32,
    gql_ocf_cnt_ch02: i32,
    gql_ocf_cnt_ch03: i32,
    gql_ocf_cnt_ch04: i32,
    gql_ocf_cnt_ch05: i32,
    gql_ocf_cnt_ch06: i32,
    gql_ocf_cnt_ch07: i32,
    gql_ocf_cnt_ch08: i32,
    gql_ocf_cnt_ch09: i32,
    gql_ocf_cnt_ch10: i32,
    gql_ocf_cnt_ch11: i32,
    gql_ocf_cnt_ch12: i32,
    gql_ocf_cnt_ch13: i32,
    gql_ocf_cnt_ch14: i32,
    gql_ocf_cnt_ch15: i32,
    gql_ocf_cnt_ch16: i32,
}
// Translation from Output to GraphQLOutput
impl GqlOverCurrentFaultState {
    pub fn from_ocf_state(ocf_state: &OverCurrentFaultState) -> Self {
        GqlOverCurrentFaultState {
            gql_stat_ch_on: ocf_state.stat_ch_on as i32,
            gql_stat_ch_ext_on: ocf_state.stat_ch_ext_on as i32,
            gql_stat_ch_ocf: ocf_state.stat_ch_ocf as i32,
            gql_stat_ch_ext_ocf: ocf_state.stat_ch_ext_ocf as i32,
            gql_ocf_cnt_ch00: ocf_state.ocf_cnt_ch00 as i32,
            gql_ocf_cnt_ch01: ocf_state.ocf_cnt_ch01 as i32,
            gql_ocf_cnt_ch02: ocf_state.ocf_cnt_ch02 as i32,
            gql_ocf_cnt_ch03: ocf_state.ocf_cnt_ch03 as i32,
            gql_ocf_cnt_ch04: ocf_state.ocf_cnt_ch04 as i32,
            gql_ocf_cnt_ch05: ocf_state.ocf_cnt_ch05 as i32,
            gql_ocf_cnt_ch06: ocf_state.ocf_cnt_ch06 as i32,
            gql_ocf_cnt_ch07: ocf_state.ocf_cnt_ch07 as i32,
            gql_ocf_cnt_ch08: ocf_state.ocf_cnt_ch08 as i32,
            gql_ocf_cnt_ch09: ocf_state.ocf_cnt_ch09 as i32,
            gql_ocf_cnt_ch10: ocf_state.ocf_cnt_ch10 as i32,
            gql_ocf_cnt_ch11: ocf_state.ocf_cnt_ch11 as i32,
            gql_ocf_cnt_ch12: ocf_state.ocf_cnt_ch12 as i32,
            gql_ocf_cnt_ch13: ocf_state.ocf_cnt_ch13 as i32,
            gql_ocf_cnt_ch14: ocf_state.ocf_cnt_ch14 as i32,
            gql_ocf_cnt_ch15: ocf_state.ocf_cnt_ch15 as i32,
            gql_ocf_cnt_ch16: ocf_state.ocf_cnt_ch16 as i32,
        }
    }
}


//ABFState
#[derive(Serialize, Deserialize)]
pub struct GqlABFState {
    gql_abf_placed_0: bool,
    gql_abf_placed_1: bool,
}

impl GqlABFState {
    pub fn from_abf_state(abf_state: &ABFState) -> Self {
        GqlABFState {
            gql_abf_placed_0: abf_state.abf_placed_0 == 0xAB,
            gql_abf_placed_1: abf_state.abf_placed_1 == 0xAB,
        }
    }
}

//GqlVIPData
#[derive(Serialize, Deserialize)]
pub struct GqlVIPData {
    gql_volt: f64,
    gql_curr: f64,
    gql_pwr: f64,
}

impl GqlVIPData {
    pub fn from_vip_data(vip_data: &VIPData) -> Self {
        GqlVIPData {
            gql_volt: vip_data.volt as f64,
            gql_curr: vip_data.curr as f64,
            gql_pwr: vip_data.pwr as f64,
        }
    }
}

//GqlCondChnShortData
#[derive(Serialize, Deserialize)]
pub struct GqlCondChnShortData {
    gql_volt_in_mppt: f64,
    gql_curr_in_mppt: f64,
    gql_volt_out_mppt: f64,
    gql_curr_out_mppt: f64,
}

impl GqlCondChnShortData {
    pub fn from_cond_chn_short_data(cond_chn_short_data: &CondChnShortData) -> Self {
        GqlCondChnShortData {
            gql_volt_in_mppt: cond_chn_short_data.volt_in_mppt as f64,
            gql_curr_in_mppt: cond_chn_short_data.curr_in_mppt as f64,
            gql_volt_out_mppt: cond_chn_short_data.volt_out_mppt as f64,
            gql_curr_out_mppt: cond_chn_short_data.curr_out_mppt as f64,
        }
    }
}

#[derive(Serialize, Deserialize)]
// #[derive(GraphQLObject)]
pub struct GqlPIUHk {
    // One reseved byte. Starting from the 6th byte
    // Voltage of internal board supply.
    gql_volt_brdsup: i32,
    // Measured temperature of the MCU
    gql_temp: i32,
    // Input V, I and P input of the distribution part of the unit in raw form.
    gql_vip_dist_input: GqlVIPData,
    // Input V, I and P input of the battery part of the unit 
    gql_vip_batt_input: GqlVIPData,
    // Bitflag field indicating channel-on status for output 0 through 15.
    gql_stat_ch_on:i32,
    // Bitflag field indicating overcurrent latch-off fault for output 0 through 15.
    gql_stat_ch_ocf:i32,
    // Bitflag field indicating BP board status.
    gql_batt_stat:i32,
    // 2 and 4 cell battery pack
    gql_batt_temp2:i32,
    // 2 cell battery pack not used, temp for 4 cell battery pack:
    gql_batt_temp3:i32,
    // Voltage level for domain 0 - 2
    gql_volt_vd0:i32,
    gql_volt_vd1:i32,
    gql_volt_vd2:i32,
    // GqlVIPData output for channel 0 - 16
    // VD0_0, 3.3V
    gql_vip_cnt_ch00: GqlVIPData,
    // VD1_0, 5V
    gql_vip_cnt_ch01: GqlVIPData,
    // VD1_1, 5V
    gql_vip_cnt_ch02: GqlVIPData,
    // VD1_2, 5V
    gql_vip_cnt_ch03: GqlVIPData,
    // VD1_3, 3.3V
    gql_vip_cnt_ch04: GqlVIPData,
    // VD2_0, 3.3V
    gql_vip_cnt_ch05: GqlVIPData,
    // VD2_1, 3.3V
    gql_vip_cnt_ch06: GqlVIPData,
    // VD2_2, 3.3V
    gql_vip_cnt_ch07: GqlVIPData,
    // VD2_3, 3.3V
    gql_vip_cnt_ch08: GqlVIPData,
    // Data on conditioning chain
    gql_ccd1: GqlCondChnShortData,
    gql_ccd2: GqlCondChnShortData,
    gql_ccd3: GqlCondChnShortData,
    // VD0_1, 3.3V 
    gql_vip_cnt_ch09: GqlVIPData,
    // VD0_2, 3.3V    
    gql_vip_cnt_ch10: GqlVIPData,
    // VD0_3, 3.3V    
    gql_vip_cnt_ch11: GqlVIPData,
    // VD3_0, 5.4V (customized)
    gql_vip_cnt_ch12: GqlVIPData,
    // VD3_1, 5.4V (customized)
    gql_vip_cnt_ch13: GqlVIPData,
    // VD4_0, 12V (customized)
    gql_vip_cnt_ch14: GqlVIPData,
    // VD4_1, 12V (customized)
    gql_vip_cnt_ch15: GqlVIPData,
    // Data on conditioning chain 
    gql_ccd4: GqlCondChnShortData,
    gql_ccd5: GqlCondChnShortData,
    // Bitflag field indicating channel-on status for the extended output bus channels
    gql_stat_ch_ext_on: i32,
    // Bitflag field indicating overcurrent latch-off fault status for the extended output bus channels
    gql_stat_ch_ext_ocf: i32,
    // VD5_0, 28.2V (default)
    gql_vip_cnt_ch16: i32,
    // Stop at 184 byte for the ICEPSv2
}

impl GqlPIUHk {
    pub fn from_piu_hk(piu_hk: &PIUHk) -> Self {
        GqlPIUHk {
            gql_volt_brdsup: piu_hk.volt_brdsup as f64,
            gql_temp: piu_hk.temp as f64,
            gql_vip_dist_input: GqlVIPData::from_vip_data(&piu_hk.vip_dist_input),
            gql_vip_batt_input: GqlVIPData::from_vip_data(&piu_hk.vip_batt_input),
            gql_stat_ch_on: piu_hk.stat_ch_on as f64,
            gql_stat_ch_ocf: piu_hk.stat_ch_ocf as f64,
            gql_batt_stat: piu_hk.batt_stat as f64,
            gql_batt_temp2: piu_hk.batt_temp2 as f64,
            gql_batt_temp3: piu_hk.batt_temp3 as f64,
            gql_volt_vd0: piu_hk.volt_vd0 as f64,
            gql_volt_vd1: piu_hk.volt_vd1 as f64,
            gql_volt_vd2: piu_hk.volt_vd2 as f64,
            gql_vip_cnt_ch00: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch00),
            gql_vip_cnt_ch01: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch01),
            gql_vip_cnt_ch02: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch02),
            gql_vip_cnt_ch03: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch03),
            gql_vip_cnt_ch04: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch04),
            gql_vip_cnt_ch05: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch05),
            gql_vip_cnt_ch06: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch06),
            gql_vip_cnt_ch07: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch07),
            gql_vip_cnt_ch08: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch08),
            gql_ccd1: GqlCondChnShortData::from_cond_chn_short_data(&piu_hk.ccd1),
            gql_ccd2: GqlCondChnShortData::from_cond_chn_short_data(&piu_hk.ccd2),
            gql_ccd3: GqlCondChnShortData::from_cond_chn_short_data(&piu_hk.ccd3),
            gql_vip_cnt_ch09: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch09),
            gql_vip_cnt_ch10: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch10),
            gql_vip_cnt_ch11: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch11),
            gql_vip_cnt_ch12: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch12),
            gql_vip_cnt_ch13: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch13),
            gql_vip_cnt_ch14: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch14),
            gql_vip_cnt_ch15: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch15),
            gql_ccd4: GqlCondChnShortData::from_cond_chn_short_data(&piu_hk.ccd4),
            gql_ccd5: GqlCondChnShortData::from_cond_chn_short_data(&piu_hk.ccd5),
            gql_stat_ch_ext_on: piu_hk.stat_ch_ext_on as i32,
            gql_stat_ch_ext_ocf: piu_hk.stat_ch_ext_ocf as i32,
            gql_vip_cnt_ch16: GqlVIPData::from_vip_data(&piu_hk.vip_cnt_ch16),
        }
    }
}
