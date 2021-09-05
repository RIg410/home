use crate::infra::mtrf_infra::mtrf_wire::{Info, OnMsg, CH};
use std::collections::HashMap;

pub mod mtrf_wire;

#[derive(Default)]
pub struct MtrfInfo {
    pub devs: HashMap<CH, (Box<dyn OnMsg>, Info)>,
}
