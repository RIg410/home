use hap::accessory::switch::SwitchAccessory;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn next_id() -> u64 {
    static GEN: AtomicU64 = AtomicU64::new(1);
    GEN.fetch_add(1, Ordering::Relaxed)
}

pub enum Accessory {
    SwitchAccessory(SwitchAccessory),
}
