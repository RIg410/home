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
pub struct Switch {
    name: &'static str,
    info_url: Url,
    update_url: Url,
}

impl Switch {
    pub fn new(name: &'static str) -> Result<Switch> {
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
        })
    }

    pub fn is_on(&self) -> Result<bool> {
        let state: State = get(self.info_url.clone())?.json()?;
        Ok(state.is_on)
    }

    pub fn update_state(&self, is_on: bool) -> Result<()> {
        let resp = Client::builder()
            .build()?
            .post(self.update_url.clone())
            .json(&State { is_on })
            .send()?;
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
struct State {
    pub is_on: bool,
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
                "http://192.168.0.100:{}/odin/api/switch/{}/toggle",
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