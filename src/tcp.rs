pub struct State {}

impl Default for State {
    fn default() -> Self {
        todo!()
    }
}

impl State {
    pub fn on_packet<'a>(
        &'a self,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        todo!()
    }
}
