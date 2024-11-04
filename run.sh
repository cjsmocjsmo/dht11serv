
systemctl stop dht11serv
cd /usr/share/dht11serv/dht11serv
git pull
cargo build --release
systemctl start dht11serv