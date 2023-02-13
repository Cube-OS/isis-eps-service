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

// Input/Output GQL Structs for conversion 

// Only Inputs needed for conversion
#[derive(GraphQLEnum,Clone)]
pub enum GqlStID {
    // Power Distribution Unit System Type Identifier
    GqlPduStid,
    // Power Battery Unit System Type Identifier
    GqlPbuStid,
    // Power Condition Unit System Type Identifier
    GqlPcuStid,
    // Power Intergration unit System Type Identifier
    GqlPiuStid,
    // over write System Type Identifier (i.e. Stid = 0x00)
    GqlOverrideStid,
}
impl From<isis_eps_api::StID> for GqlStID {
    fn from(t: isis_eps_api::StID) -> GqlStID {
        match t {
            isis_eps_api::StID::PduStid => GqlStID::GqlPduStid,
            isis_eps_api::StID::PbuStid => GqlStID::GqlPbuStid,
            isis_eps_api::StID::PcuStid => GqlStID::GqlPcuStid,
            isis_eps_api::StID::PiuStid => GqlStID::GqlPiuStid,      
            isis_eps_api::StID::OverrideStid => GqlStID::GqlOverrideStid,                  
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlPDUHkSel {
    // PDURawHK,
    GqlPDUEngHK,
    GqlPDUAvgHK,
}
impl From<isis_eps_api::PDUHkSel> for GqlPDUHkSel {
    fn from(t: isis_eps_api::PDUHkSel) -> GqlPDUHkSel {
        match t {
            isis_eps_api::PDUHkSel::PDUEngHK => GqlPDUHkSel::GqlPDUEngHK,
            isis_eps_api::PDUHkSel::PDUAvgHK => GqlPDUHkSel::GqlPDUAvgHK,
        }
    }
}


pub enum GqlPBUHkSel {
    // PBURawHK,
    GqlPBUEngHK,
    GqlPBUAvgHK,
}
impl From<isis_eps_api::PBUHkSel> for GqlPBUHkSel {
    fn from(t: isis_eps_api::PBUHkSel) -> GqlPBUHkSel {
        match t {
            isis_eps_api::PBUHkSel::PBUEngHK => GqlPBUHkSel::GqlPBUEngHK,
            isis_eps_api::PBUHkSel::PBUAvgHK => GqlPBUHkSel::GqlPBUAvgHK,
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlPCUHkSel {
    // PCURawHK,
    GqlPCUEngHK,
    GqlPCUAvgHK,
}
impl From<isis_eps_api::PCUHkSel> for GqlPCUHkSel {
    fn from(t: isis_eps_api::PCUHkSel) -> GqlPCUHkSel {
        match t {
            isis_eps_api::PCUHkSel::PCUEngHK => GqlPCUHkSel::GqlPCUEngHK,
            isis_eps_api::PCUHkSel::PCUAvgHK => GqlPCUHkSel::GqlPCUAvgHK,
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlPIUHkSel {
    // PIURawHK,
    GqlPIUEngHK,
    GqlPIUAvgHK, 
}
impl From<isis_eps_api::PIUHkSel> for GqlPIUHkSel {
    fn from(t: isis_eps_api::PIUHkSel) -> GqlPIUHkSel {
        match t {
            isis_eps_api::PIUHkSel::PIUEngHK => GqlPIUHkSel::GqlPIUEngHK,
            isis_eps_api::PIUHkSel::PIUAvgHK => GqlPIUHkSel::GqlPIUAvgHK,
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlBusGroup {
    GqlBusGroupOn,
    GqlBusGroupOff,
    GqlBusGroupState,
}
impl From<isis_eps_api::BusGroup> for GqlBusGroup {
    fn from(t: isis_eps_api::BusGroup) -> GqlBusGroup {
        match t {
            isis_eps_api::BusGroup::BusGroupOn => GqlBusGroup::GqlBusGroupOn,
            isis_eps_api::BusGroup::BusGroupOff => GqlBusGroup::GqlBusGroupOff,
            isis_eps_api::BusGroup::BusGroupState => GqlBusGroup::GqlBusGroupState,
        }
    }
}

// Output GqlBus Channel
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[derive(GraphQLEnum,Clone)]
pub enum GqlBusChannel {
    GqlChannelOn,
    GqlChannelOff,
}
impl From<isis_eps_api::BusChannel> for GqlBusChannel {
    fn from(t: isis_eps_api::BusChannel) -> GqlBusChannel {
        match t {
            isis_eps_api::BusChannel::ChannelOn => GqlBusChannel::GqlChannelOn,
            isis_eps_api::BusChannel::ChannelOff => GqlBusChannel::GqlChannelOff,
        }
    }
}


// Used in ModeSwitch (0x30/0x31)
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[derive(GraphQLEnum,Clone)]
pub enum GqlModeSwitch {
    GqlNominal,
    GqlSafety,
}
impl From<isis_eps_api::ModeSwitch> for GqlModeSwitch {
    fn from(t: isis_eps_api::ModeSwitch) -> GqlModeSwitch {
        match t {
            isis_eps_api::ModeSwitch::Nominal => GqlModeSwitch::GqlNominal,
            isis_eps_api::ModeSwitch::Safety => GqlModeSwitch::GqlSafety,
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlSysConfig1 {
    GqlGetConfigParam, 
    GqlSetConfigParam, 
    GqlResetConfigParam,
}
impl From<isis_eps_api::SysConfig1> for GqlSysConfig1 {
    fn from(t: isis_eps_api::SysConfig1) -> GqlSysConfig1 {
        match t {
            isis_eps_api::SysConfig1::GetConfigParam => GqlSysConfig1::GqlGetConfigParam,
            isis_eps_api::SysConfig1::SetConfigParam => GqlSysConfig1::GqlSetConfigParam,
            isis_eps_api::SysConfig1::ResetConfigParam => GqlSysConfig1::GqlResetConfigParam,
        }
    }
}

#[derive(GraphQLEnum,Clone)]
pub enum GqlSysConfig2 {
    GqlResetAll, 
    GqlLoadConfig, 
    GqlSaveConfig, 
}
impl From<isis_eps_api::SysConfig2> for GqlSysConfig2 {
    fn from(t: isis_eps_api::SysConfig2) -> GqlSysConfig2 {
        match t {
            isis_eps_api::SysConfig2::ResetAll => GqlSysConfig2::GqlResetAll,
            isis_eps_api::SysConfig2::LoadConfig => GqlSysConfig2::GqlLoadConfig,
            isis_eps_api::SysConfig2::SaveConfig => GqlSysConfig2::GqlSaveConfig,
        }
    }
}
// OUTPUT
