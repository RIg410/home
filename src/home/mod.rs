use crate::home::handlers::Handlers;
use crate::home::rooms::bedroom::Bedroom;
use crate::home::rooms::hall::Hall;
use anyhow::Result;
use std::sync::Arc;

pub mod handlers;
pub mod rooms;

#[derive(Debug)]
pub struct Home {
    bedroom: Bedroom,
    hall: Hall,
}

impl Home {}

pub fn init() -> Result<(Arc<Home>, Handlers)> {
    let mut handlers = Handlers::default();
    let home = Home {
        bedroom: Bedroom::new(&mut handlers)?,
        hall: Hall::new(&mut handlers)?,
    };

    Ok((Arc::new(home), handlers))
}
