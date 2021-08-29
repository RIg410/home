use crate::infra::mtrf_wire::{OnMsg, CH};
use std::collections::HashMap;

#[derive(Default)]
pub struct Handlers {
    mtrf: HashMap<CH, Box<dyn OnMsg>>,
}

impl Handlers {
    pub fn mtrf<M: OnMsg + Clone>(&mut self, dev: M) -> M {
        self.mtrf.insert(dev.ch(), Box::new(dev.clone()));
        dev
    }

    pub fn into_inner(self) -> HashMap<CH, Box<dyn OnMsg>> {
        self.mtrf
    }
}
