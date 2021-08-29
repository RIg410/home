use crate::home::rooms::bedroom::Bedroom;
use crate::home::Home;

pub mod bedroom;

pub trait Room {
    fn this(home: &Home) -> &Self;
}

impl Room for Bedroom {
    fn this(home: &Home) -> &Self {
        &home.bedroom
    }
}
