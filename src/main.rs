use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use tcp_protocol::tcp;

extern crate tun_tap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}

fn main() -> std::io::Result<()> {
    let connections: HashMap<Quad, tcp::State> = HashMap::new();
    // Create the Tun (logical NIC)
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer: [u8; 1504] = [0; 1504];
    loop {
        let bytes = nic.recv(&mut buffer[..])?;
        if bytes < 4 {
            break;
        }
        // for the ethernet frame
        let _eth_flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto = u16::from_be_bytes([buffer[2], buffer[3]]);

        if eth_proto != 0x0800 {
            // Ignore packet if not an IPv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4..bytes]) {
            Ok(packet) => {
                let payload_len = match packet.payload_len() {
                    Ok(len) => len,
                    Err(_) => continue,
                };

                // IP level protocol
                let protocol: u8 = packet.protocol().0;

                if protocol != 6 {
                    // Not TCP
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buffer[4 + packet.slice().len()..]) {
                    Ok(p) => {
                        let source: Ipv4Addr = Ipv4Addr::from(packet.source_addr());
                        let destination: Ipv4Addr = Ipv4Addr::from(packet.destination_addr());
                        let port = p.destination_port();

                        eprintln!(
                            "{} -> {} {} bytes of tcp to port {}",
                            source, destination, payload_len, port
                        );
                    }
                    Err(_) => {
                        eprintln!("Ignoring weird tcp packet");
                        continue;
                    }
                }
            }
            Err(_) => {
                eprintln!("Ignore weird packet");
            }
        };
    }
    Ok(())
}
