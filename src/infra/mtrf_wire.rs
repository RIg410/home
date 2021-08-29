use crate::home::Home;
use anyhow::Result;
use mtrf::cmd::request::Request;
use mtrf::cmd::response::Response;
use mtrf::cmd::{Cmd, CtrResponse, Mode};
use mtrf::mtrf::{Mtrf, OnMessage};
use mtrf::ports;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub type CH = u8;

static INSTANCE: OnceCell<Mutex<Mtrf>> = OnceCell::new();

pub fn send(req: Request) -> Result<()> {
    if let Some(mtrf) = INSTANCE.get() {
        mtrf.lock().send(req)
    } else {
        bail!("Failed to send mtrf request. The mtrf is not initialized.");
    }
}

pub fn send_request(req: Request) -> Result<Response> {
    if let Some(mtrf) = INSTANCE.get() {
        mtrf.lock().send_request(req)
    } else {
        bail!("Failed to send mtrf request. The mtrf is not initialized.")
    }
}

pub trait OnMsg: Send + 'static {
    fn ch(&self) -> CH;
    fn on_msg(&self, home: &Home, msg: Message);
}

struct MessageHandler {
    handlers: HashMap<CH, Box<dyn OnMsg>>,
    home: Arc<Home>,
}

impl OnMessage for MessageHandler {
    fn on_message(&mut self, msg: Response) {
        if let Some(hdl) = self.handlers.get(&msg.ch) {
            hdl.on_msg(self.home.as_ref(), msg.into());
        } else {
            warn!("No handlers found for msg: {}", msg);
        }
    }
}

pub fn init(handlers: HashMap<CH, Box<dyn OnMsg>>, home: Arc<Home>) -> Result<()> {
    let ports = ports()?;
    if ports.is_empty() {
        error!("Failed to init mtrf. No ports were found.");
        return Ok(());
    }
    let port = &ports[0];
    info!("Init mrtf using port:{}", port.port_name);

    let mtrf = Mtrf::new(port, MessageHandler { handlers, home })?;
    INSTANCE
        .set(Mutex::new(mtrf))
        .map_err(|_| anyhow!("Mtrf is already initialized."))?;
    Ok(())
}

#[derive(Debug)]
pub struct Message {
    pub mode: Mode,
    pub ctr: CtrResponse,
    pub togl: u8,
    pub cmd: Cmd,
    pub id: u32,
}

impl From<Response> for Message {
    fn from(resp: Response) -> Self {
        Message {
            mode: resp.mode,
            ctr: resp.ctr,
            togl: resp.togl,
            cmd: resp.cmd,
            id: resp.id,
        }
    }
}
