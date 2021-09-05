use crate::home::Home;
use crate::infra::mtrf_infra::MtrfInfo;
use crate::infra::telegram::{TBot, User};
use anyhow::Result;
use mtrf::cmd::request::Request;
use mtrf::cmd::response::Response;
use mtrf::cmd::{Cmd, CtrResponse, Mode};
use mtrf::mtrf::{Mtrf, OnMessage};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serialport::SerialPortType;
use std::collections::HashMap;
use std::sync::Arc;

pub type CH = u8;

const SERIAL: &str = "AL065KM0";

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

#[derive(Debug)]
pub struct Info {
    pub loc: &'static str,
    pub name: &'static str,
}

struct MessageHandler {
    handlers: HashMap<CH, (Box<dyn OnMsg>, Info)>,
    home: Arc<Home>,
    bot: TBot,
}

impl OnMessage for MessageHandler {
    fn on_message(&mut self, msg: Response) {
        if let Some((hdl, info)) = self.handlers.get(&msg.ch) {
            if let Cmd::BatteryLow = msg.cmd {
                if let Err(err) = self.bot.send_msg(
                    User::Root,
                    format!(
                        "The {} device located in the {} is discharged. 😭",
                        info.loc, info.name
                    ),
                ) {
                    error!(
                        "Failed to send low energy message about {:?}. Error:[{:?}]",
                        info, err
                    );
                }
            }

            hdl.on_msg(self.home.as_ref(), msg.into());
        } else {
            warn!("No handlers found for msg: {}", msg);
        }
    }
}

pub fn init(mtrf: MtrfInfo, bot: TBot, home: Arc<Home>) -> Result<()> {
    let serial = Some(SERIAL.to_string());
    let ports = serialport::available_ports()?
        .into_iter()
        .filter_map(|p| match p.port_type {
            SerialPortType::UsbPort(usb) => {
                if usb.serial_number == serial || p.port_name.contains(SERIAL) {
                    Some(p.port_name)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if ports.is_empty() {
        error!("Failed to init mtrf. No ports were found.");
        return Ok(());
    }
    let port = &ports[0];
    info!("Init mrtf using port:{}", port);

    let mtrf = Mtrf::new(
        port,
        MessageHandler {
            handlers: mtrf.devs,
            home,
            bot,
        },
    )?;
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
