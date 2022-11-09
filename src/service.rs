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
// The service.rs file is the core of each service
// It enables the communication via UDP or GraphQL (depending on --features flag during compilation)

use cubeos_service::*;
use isis_eps_api::*;  

// #[cfg(not(feature = "ground"))]
// use crate::subsystem::*;

#[cfg(feature = "ground")]
use crate::graphql::*;

// Macro to create UDP-handler function or GraphQL Queries and Mutations
// The layout follows the rules:
// query/mutation: Command-Name => Function as defined in subsystem.rs; in: GraphQLInput; out: GraphQLOutput;
//
// GraphQLInput is only needed if Input is a struct that contains fields with types other than i32,f64,String or bool
// GraphQLOutput is only needed if the Output should be formatted in humanly readable way
// (e.g. Payload telemetry returns a Vec<u8>, but resembles analog data like Voltage,Current,Temperature etc.)
// If GraphQLInput/Output are not needed then please set to Input and Output of function

// Note that Service Ping, GetLastError, GetLastMutation are already inculded in the CubeOS-Service           
service_macro! {
    subsystem::Subsystem{ 
        query: EpsPing => fn eps_ping(&self, typ_id: StID) -> Result<()>; in: StID;
        query: SystemStatus => fn sys_reset(&self, typ_id: StID, ret_key: u8) -> Result<()>; in: StID, i32;
        query: OvercurrentState => fn overcurrent_state(&self, typ_id: StID) -> Result<Vec<u8>>; in: StID; out: Vec<u8>;
        query: AbfState => fn abf_state(&self, typ_id: StID) -> Result<Vec<u8>>; in: StID; out: Vec<u8>;
        query: PduHk => fn pdu_hk(&self, typ_stid: StID, mode: PDUHkSel) -> Result<Vec<u8>>; in: StID, PDUHkSel; out: Vec<u8>;
        query: PbuHk => fn pbu_hk(&self, typ_stid: StID, mode: PBUHkSel) -> Result<Vec<u8>>; in: StID, PBUHkSel; out: Vec<u8>;
        query: PiuHk => fn piu_hk(&self, typ_stid: StID, mode: PIUHkSel) -> Result<Vec<u8>>; in: StID, PIUHkSel; out: Vec<u8>;
        query: PcuHk => fn pcu_hk(&self, typ_stid: StID, mode: PCUHkSel) -> Result<Vec<u8>>; in: StID, PCUHkSel; out: Vec<u8>;
        mutation: SysReset => fn sys_reset(&self, typ_id: StID, ret_key: u8) -> Result<()>; in: StID, i32;
        mutation: ShutDownAll =>fn shutdown_all(&self, typ_id: StID) -> EpsResult<()>; in: StID;
        mutation: WatchdogReset => fn watchdog_reset(&self, typ_id: StID) -> EpsResult<()>; in: StID;
        mutation: SetGroupOutputs => fn set_group_outputs(&self, typ_id: StID, typ_group: BusGroup, eps_bitflag: u16) -> EpsResult<()>; in: StID, BusGroup, i32;
        mutation: SetSingleOutput => fn set_single_output(&self, typ_stid: StID, typ_channel: BusChannel, eps_ch_idx: u8) -> EpsResult<()>; in: StID, BusChannel, i32;
        mutation: ModeSwitch => fn mode_switch(&self, typ_stid: StID, mode: ModeSwitch) -> EpsResult<()>; in: StID, ModeSwitch;
        mutation: SystemConfigCmd => fn system_config_cmd(&self, typ_stid: StID, mode: SysConfig1, para_id: u16) -> EpsResult<Vec<u8>>; in: StID, SysConfig1, i32; out: Vec<u8>;
        mutation: ResetAllConf => fn reset_all_conf(&self, typ_stid: StID, mode: SysConfig2, config_key: u8) -> EpsResult<()>; in: StID, SysConfig2, i32;
        mutation: CorrectTime => fn correct_time(&self, typ_stid: StID, time_correction: i32) -> EpsResult<()>; in: StID, i32;
        mutation: ResetAllCounters => fn reset_all_counters(&self, typ_stid: StID, zero_key: u8) -> EpsResult<()>; in: StID, i32;
    }
}
