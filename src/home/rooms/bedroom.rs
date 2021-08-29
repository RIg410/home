use crate::devices::mtrf_dev::Switch;
use crate::devices::proxy::switch::Switch as ProxySwitch;
use crate::home::handlers::Handlers;
use crate::home::Home;
use crate::infra::mtrf_wire::CH;
use anyhow::Result;
use crate::home::rooms::Room;

const LEFT_SWITCH_ID: CH = 0;
const RIGHT_SWITCH_ID: CH = 1;

#[derive(Debug)]
pub struct Bedroom {
    pub lamp: ProxySwitch,
    // pub curtain: CurtainSwitch,
    pub left_switch: Switch,
    pub right_switch: Switch,
}

impl Bedroom {
    pub fn new(hdlr: &mut Handlers) -> Result<Bedroom> {
        Ok(Bedroom {
            lamp: ProxySwitch::new("bedroom_lamp")?,
            left_switch: hdlr.mtrf(Switch::on_switch(LEFT_SWITCH_ID, Box::new(left_switch))),
            right_switch: hdlr.mtrf(Switch::on_switch(RIGHT_SWITCH_ID, Box::new(right_switch))),
        })
    }
}

fn left_switch(home: &Home) -> Result<()> {
    println!("left_switch");
    Ok(())
}

fn right_switch(home: &Home) -> Result<()> {
    Bedroom::this(home).lamp.switch()
}
