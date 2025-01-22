cargo b --release
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp_protocol
$CARGO_TARGET_DIR/release/tcp_protocol &
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
fg
