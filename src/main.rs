extern crate tun_tap;

fn main() -> std::io::Result<()> {
    // Create the Tun
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer: [u8; 1504] = [0; 1504];
    let bytes = nic.recv(&mut buffer[..])?;
    eprintln!("Read {} bytes into buffer: {:?}", bytes, &buffer[..bytes]);
    Ok(())
}
