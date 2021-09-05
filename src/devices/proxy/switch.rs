use crate::devices::proxy::OLD_SERVICE_PORT;
use crate::infra::hap_infra::{next_id, Accessory};
use anyhow::{Error, Result};
use hap::accessory::switch::SwitchAccessory;
use hap::accessory::AccessoryInformation;
use hap::characteristic::CharacteristicCallbacks;
use reqwest::blocking::{get, Client};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum SwitchType {
    Light,
    Curtain,
}

#[derive(Debug, Clone)]
pub struct Switch {
    name: &'static str,
    info_url: Url,
    update_url: Url,
    tp: SwitchType,
}

impl Switch {
    pub fn new(name: &'static str, tp: SwitchType) -> Result<Switch> {
        Ok(Switch {
            name,
            info_url: Url::parse(&format!(
                "http://localhost:{}/odin/api/v1/device/{}/info",
                OLD_SERVICE_PORT, name
            ))?,
            update_url: Url::parse(&format!(
                "http://localhost:{}/odin/api/v1/device/{}/update",
                OLD_SERVICE_PORT, name
            ))?,
            tp,
        })
    }

    pub fn is_on(&self) -> Result<bool> {
        match self.tp {
            SwitchType::Light => {
                let state: LightState = get(self.info_url.clone())?.json()?;
                Ok(state.is_on)
            }
            SwitchType::Curtain => {
                let state: CurtainState = get(self.info_url.clone())?.json()?;
                Ok(state.is_open)
            }
        }
    }

    pub fn update_state(&self, is_on: bool) -> Result<()> {
        let mut builder = Client::builder().build()?.post(self.update_url.clone());
        match self.tp {
            SwitchType::Light => {
                builder = builder.json(&LightState { is_on });
            }
            SwitchType::Curtain => {
                builder = builder.json(&CurtainState { is_open: is_on });
            }
        };

        let resp = builder.send()?;
        if resp.status().is_success() {
            Ok(())
        } else {
            bail!("Failed to change {} state. {}", self.name, resp.status())
        }
    }

    pub fn switch(&self) -> Result<()> {
        self.update_state(!self.is_on()?)
    }
}

impl TryFrom<Switch> for Accessory {
    type Error = Error;

    fn try_from(value: Switch) -> Result<Self> {
        let mut switch = SwitchAccessory::new(
            next_id(),
            AccessoryInformation {
                name: value.name.to_owned(),
                ..Default::default()
            },
        )?;
        {
            let this = value.clone();
            switch
                .switch
                .power_state
                .on_read(Some(move || Ok(Some(this.is_on()?))));
        }

        {
            let this = value;
            switch
                .switch
                .power_state
                .on_update(Some(move |_: &bool, new_val: &bool| {
                    this.update_state(*new_val)?;
                    Ok(())
                }));
        }
        Ok(Accessory::SwitchAccessory(switch))
    }
}

#[derive(Serialize, Deserialize)]
struct LightState {
    pub is_on: bool,
}

#[derive(Serialize, Deserialize)]
struct CurtainState {
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Toggle {
    name: &'static str,
    url: Url,
}

impl Toggle {
    pub fn new(name: &'static str) -> Result<Toggle> {
        Ok(Toggle {
            name,
            url: Url::parse(&format!(
                "http://localhost:{}/odin/api/switch/{}/toggle",
                OLD_SERVICE_PORT, name
            ))?,
        })
    }

    pub fn toggle(&self) -> Result<()> {
        let resp = get(self.url.clone())?;
        if resp.status().is_success() {
            Ok(())
        } else {
            bail!("Failed to change {} state. {}", self.name, resp.status())
        }
    }
}
