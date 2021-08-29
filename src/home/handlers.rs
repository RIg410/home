use crate::infra::hap_infra::Accessory;
use crate::infra::mtrf_wire::{OnMsg, CH};
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Default)]
pub struct Handlers {
    mtrf: HashMap<CH, Box<dyn OnMsg>>,
    hap: Vec<Accessory>,
}

impl Handlers {
    pub fn mtrf<M: OnMsg + Clone>(&mut self, dev: M) -> M {
        self.mtrf.insert(dev.ch(), Box::new(dev.clone()));
        dev
    }

    pub fn hap<H: TryInto<Accessory, Error = Error> + Clone>(&mut self, dev: H) -> Result<H> {
        self.hap.push(dev.clone().try_into()?);
        Ok(dev)
    }

    pub fn into_inner(self) -> HashMap<CH, Box<dyn OnMsg>> {
        self.mtrf
    }
}
