use anyhow::Result;
use reqwest::blocking::{get, Client};
use reqwest::{Url, StatusCode};
use crate::devices::proxy::OLD_SERVICE_PORT;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Switch {
    name: &'static str,
    info_url: Url,
    update_url: Url,
}

impl Switch {
    pub fn new(name: &'static str) -> Result<Switch> {
        Ok(Switch {
            name,
            info_url: Url::parse(&format!("http://localhost:{}/odin/api/v1/device/{}/info", OLD_SERVICE_PORT, name))?,
            update_url: Url::parse(&format!("http://localhost:{}/odin/api/v1/device/{}/update", OLD_SERVICE_PORT, name))?,
        })
    }

    pub fn is_on(&self) -> Result<bool> {
        let state: State = get(self.info_url.clone())?.json()?;
        Ok(state.is_on)
    }

    pub fn update_state(&self, is_on: bool) -> Result<()> {
        let resp = Client::builder().build()?
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


#[derive(Serialize, Deserialize)]
struct State {
    pub is_on: bool,
}