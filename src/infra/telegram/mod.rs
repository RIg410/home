use anyhow::Error;

pub struct TBot {}

impl TBot {
    pub fn new() -> Result<TBot, Error> {
        Ok(TBot {})
    }

    pub fn send_msg(&self, _usr: User, _msg: String) -> Result<(), Error> {
        Ok(())
    }
}

pub enum User {
    Root,
}
