use crate::home::Home;
use crate::infra::mtrf_infra::mtrf_wire::{Message, OnMsg, CH};
use anyhow::Result;
use mtrf::cmd::Cmd;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Switch {
    inner: Arc<Inner>,
}

impl Switch {
    pub fn on_switch(
        ch: CH,
        on_switch: Box<dyn Fn(&Home) -> Result<()> + Send + Sync + 'static>,
    ) -> Switch {
        Switch {
            inner: Arc::new(Inner {
                is_on: Default::default(),
                ch,
                switch: on_switch,
            }),
        }
    }
}

impl OnMsg for Switch {
    fn ch(&self) -> CH {
        self.inner.ch
    }

    fn on_msg(&self, home: &Home, msg: Message) {
        let res = match msg.cmd {
            Cmd::On => {
                if self
                    .inner
                    .is_on
                    .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    (self.inner.switch)(home)
                } else {
                    Ok(())
                }
            }
            Cmd::Off => {
                if self
                    .inner
                    .is_on
                    .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    (self.inner.switch)(home)
                } else {
                    Ok(())
                }
            }
            Cmd::Switch => {
                self.inner
                    .is_on
                    .store(!self.inner.is_on.load(Ordering::Relaxed), Ordering::Relaxed);
                (self.inner.switch)(home)
            }
            _ => {
                warn!("Unsupported message type.{:?}", msg);
                Ok(())
            }
        };
        if let Err(err) = res {
            error!("{}", err);
        }
    }
}

impl Debug for Switch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MtrfSwitch({}:{})",
            self.inner.ch,
            if self.inner.is_on.load(Ordering::Relaxed) {
                "on"
            } else {
                "off"
            }
        )
    }
}

struct Inner {
    is_on: AtomicBool,
    ch: CH,
    switch: Box<dyn Fn(&Home) -> Result<()> + Send + Sync + 'static>,
}
