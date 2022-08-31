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

// Input/Output GQL Structs for conversion 

// INPUT
#[derive(GraphQLInputObject,Clone)]
pub struct GqlInput {
    gql_in_timeCorrection: i32,
    gql_in_OBC_BF: u16,
    gql_in_OBC_IDX: u8,
    gql_in_ROS: u8,
}




// OUTPUT
#[derive(Serialize,Deserialize)]
pub struct GqlVIPD {
    gql_out_VOLT: f64,
    gql_out_CURR: f64,
    gql_out_POWE: f64,
}

#[derive(Serialize,Deserialize)]
pub struct GqlCCSD {
    gql_out_VOLT_IN_MPPT: f64,
    gql_out_CURR_IN_MPPT: f64,
    gql_out_VOLT_OU_MPPT: f64,
    gql_out_CURR_OU_MPPT: f64,
}
// Translation for OUTPUT
pub struct GqlOutput {
    // In byte order the first 4 (inputs) would be before the STAT.
    // Most functions only require STAT and the rest is N/A-----
    gql_out_STAT: f64,
    // reserved: u8,-----
    gql_out_VOLT_BRDSUP: f64,
    gql_out_TEMP: f64,
    gql_out_VIP_DIST_INPUT: GqlVIPD,
    gql_out_VIP_BAT_INPUT: GqlVIPD,
    gql_out_STAT_OBC_ON: f64,
    gql_out_STAT_OBC_OCF: f64,
    gql_out_BAT_STAT: f64,
    gql_out_BAT_TEMP2: f64,
    gql_out_BAT_TEMP3: f64,
    gql_out_VOLT_VD0: f64,
    gql_out_VOLT_VD1: f64,
    gql_out_VOLT_VD2: f64,
    gql_out_VIP_OBC01: GqlVIPD,
    gql_out_VIP_OBC02: GqlVIPD,
    gql_out_VIP_OBC03: GqlVIPD,
    gql_out_VIP_OBC04: GqlVIPD,
    gql_out_VIP_OBC05: GqlVIPD,
    gql_out_VIP_OBC06: GqlVIPD,
    gql_out_VIP_OBC07: GqlVIPD,
    gql_out_VIP_OBC08: GqlVIPD,
    gql_out_CC1: GqlCCSD,
    gql_out_CC2: GqlCCSD,
    gql_out_CC3: GqlCCSD,
    gql_out_VIP_OBC09: GqlVIPD,
    gql_out_VIP_OBC10: GqlVIPD,
    gql_out_VIP_OBC11: GqlVIPD,
    gql_out_VIP_OBC12: GqlVIPD,
    gql_out_VIP_OBC13: GqlVIPD,
    gql_out_VIP_OBC14: GqlVIPD,
    gql_out_CC4: GqlCCSD,
    gql_out_CC5: GqlCCSD,
}

impl From<VIPD> -> GqlVIPD {
    fn from(o: VIPD) -> GqlVIPD {
        GqlVIPD {
            gql_out_VOLT: o.VOLT,
            gql_out_CURR: o.CURR,
            gql_out_POWE: o.POWE,
        }
    }
}

impl From<CCSD> -> GqlCCSD {
    fn from(o: CCSD) -> GqlCCSD {
        GqlCCSD {
            gql_out_VOLT_IN_MPPT: o.VOLT_IN_MPPT,
            gql_out_CURR_IN_MPPT: o.CURR_IN_MPPT,
            gql_out_VOLT_OU_MPPT: o.VOLT_OU_MPPT,
            gql_out_CURR_OU_MPPT: o.CURR_OU_MPPT,
        }
    }
}

// Implement of type conversion
impl From<EPSOutput> for GqlOutput {
    
    fn from(o: EPSOutput) -> GqlOutput {
        GqlOutput{
            gql_out_STAT: o.STAT.into(),
            gql_out_TEMP: o.TEMP,
            gql_out_VIP_DIST_INPUT: from(GqlVIPD),
            gql_out_VIP_BAT_INPUT: from(GqlVIPD),
            gql_out_STAT_OBC_ON: o.STAT_OBC_ON,
            gql_out_STAT_OBC_OCF: o.STAT_OBC_OCF,
            gql_out_BAT_STAT: o.BAT_STAT,
            gql_out_BAT_TEMP2: o.BAT_TEMP2,
            gql_out_BAT_TEMP3: o.BAT_TEMP3,
            gql_out_VOLT_VD0: o.VOLT_VD0,
            gql_out_VOLT_VD1: o.VOLT_VD1,
            gql_out_VOLT_VD2: o.VOLT_VD2,
            gql_out_VIP_OBC01: from(GqlVIPD),
            gql_out_VIP_OBC02: from(GqlVIPD),
            gql_out_VIP_OBC03: from(GqlVIPD),
            gql_out_VIP_OBC04: from(GqlVIPD),
            gql_out_VIP_OBC05: from(GqlVIPD),
            gql_out_VIP_OBC06: from(GqlVIPD),
            gql_out_VIP_OBC07: from(GqlVIPD),
            gql_out_VIP_OBC08: from(GqlVIPD),
            gql_out_CC1: from(GqlCCSD),
            gql_out_CC2: from(GqlCCSD),
            gql_out_CC3: from(GqlCCSD),
            gql_out_VIP_OBC09: from(GqlVIPD),
            gql_out_VIP_OBC10: from(GqlVIPD),
            gql_out_VIP_OBC11: from(GqlVIPD),
            gql_out_VIP_OBC12: from(GqlVIPD),
            gql_out_VIP_OBC13: from(GqlVIPD),
            gql_out_VIP_OBC14: from(GqlVIPD),
            gql_out_CC4: from(GqlCCSD),
            gql_out_CC5: from(GqlCCSD),
        }
    }




}