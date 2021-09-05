use crate::devices::mtrf_dev::Switch;
use crate::devices::proxy::switch::{Switch as ProxySwitch, Toggle};
use crate::home::handlers::Handlers;
use crate::home::rooms::Room;
use crate::home::Home;
use crate::infra::mtrf_infra::mtrf_wire::CH;
use anyhow::Result;

const LOC: &str = "bedroom";
const SWITCH: &str = "switch";

const LEFT_SWITCH_ID: CH = 0;
const RIGHT_SWITCH_ID: CH = 1;

#[derive(Debug)]
pub struct Bedroom {
    pub lamp: ProxySwitch,
    pub curtain: ProxySwitch,
    pub left_switch: Switch,
    pub right_switch: Switch,
    switch_off_all: Toggle,
}

impl Bedroom {
    pub fn new(hdlr: &mut Handlers) -> Result<Bedroom> {
        Ok(Bedroom {
            lamp: hdlr.hap(ProxySwitch::new("bedroom_lamp")?)?,
            curtain: hdlr.hap(ProxySwitch::new("bad_room_curtains")?)?,
            left_switch: hdlr.mtrf(
                LOC,
                SWITCH,
                Switch::on_switch(LEFT_SWITCH_ID, Box::new(left_switch)),
            ),
            right_switch: hdlr.mtrf(
                LOC,
                SWITCH,
                Switch::on_switch(RIGHT_SWITCH_ID, Box::new(right_switch)),
            ),
            switch_off_all: Toggle::new("bedroom_1")?,
        })
    }
}

fn left_switch(home: &Home) -> Result<()> {
    Bedroom::this(home).switch_off_all.toggle()
}

fn right_switch(home: &Home) -> Result<()> {
    Bedroom::this(home).lamp.switch()
}
