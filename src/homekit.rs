use crate::infra::hap_infra::Accessory;
use anyhow::Result;
use hap::accessory::AccessoryCategory;
use hap::server::{IpServer, Server};
use hap::storage::{FileStorage, Storage};
use hap::{Config, MacAddress, Pin};
use mac_address::get_mac_address;

pub async fn init(devs: Vec<Accessory>) -> Result<()> {
    let mut storage = FileStorage::current_dir().await?;

    let addr = get_mac_address()?.ok_or_else(|| anyhow!("Failed to get mac address"))?;

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await?;
            config
        }
        Err(_) => {
            let config = Config {
                pin: pin()?,
                name: "Home".into(),
                device_id: MacAddress::new(addr.bytes()),
                category: AccessoryCategory::Bridge,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        }
    };

    let server = IpServer::new(config, storage).await?;
    info!("Starting hap.");

    for dev in devs {
        match dev {
            Accessory::SwitchAccessory(acc) => {
                server.add_accessory(acc).await?;
            }
        }
    }

    server.run_handle().await?;
    Ok(())
}

fn pin() -> Result<Pin> {
    let mut pin = [0; 8];
    for item in pin.iter_mut() {
        *item = rand::random::<u8>() % 9;
    }

    let pin = Pin::new(pin)?;
    Ok(pin)
}
