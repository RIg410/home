use crate::devices::proxy::switch::{Switch as ProxySwitch, SwitchType};
use crate::home::handlers::Handlers;
use anyhow::Result;

#[derive(Debug)]
pub struct Hall {
    pub curtain: ProxySwitch,
}

impl Hall {
    pub fn new(hdlr: &mut Handlers) -> Result<Hall> {
        Ok(Hall {
            curtain: hdlr.hap(ProxySwitch::new(
                "living_room_curtains",
                SwitchType::Curtain,
            )?)?,
        })
    }
}
