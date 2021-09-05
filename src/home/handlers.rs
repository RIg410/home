use crate::infra::hap_infra::Accessory;
use crate::infra::mtrf_infra::mtrf_wire::{Info, OnMsg};
use crate::infra::mtrf_infra::MtrfInfo;
use anyhow::{Error, Result};
use std::convert::TryInto;

#[derive(Default)]
pub struct Handlers {
    mtrf: MtrfInfo,
    hap: Vec<Accessory>,
}

impl Handlers {
    pub fn mtrf<M: OnMsg + Clone>(&mut self, loc: &'static str, name: &'static str, dev: M) -> M {
        self.mtrf
            .devs
            .insert(dev.ch(), (Box::new(dev.clone()), Info { loc, name }));
        dev
    }

    pub fn hap<H: TryInto<Accessory, Error = Error> + Clone>(&mut self, dev: H) -> Result<H> {
        self.hap.push(dev.clone().try_into()?);
        Ok(dev)
    }

    pub fn into_inner(self) -> (MtrfInfo, Vec<Accessory>) {
        (self.mtrf, self.hap)
    }
}
