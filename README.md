# home

```
sudo touch /etc/systemd/system/home.service
sudo chmod 664 /etc/systemd/system/home.service

[Unit]
Description=Home
[Service]
ExecStart=/home/pi/hcl/home
Restart=on-failure
RestartSec=5s
Environment="RUST_LOG=info"
[Install]
WantedBy=multi-user.target

sudo systemctl daemon-reload
sudo systemctl enable home

sudo systemctl start home
sudo systemctl stop home
```
