use crate::clients::directory::Client;
use sphinx::SphinxPacket;
use sphinx::route::Node as MixNode;
use tokio::prelude::*;

pub struct MixClient {}

impl MixClient {
    pub fn new() -> MixClient {
        MixClient {}
    }

    // Sends a Sphinx packet to a mixnode.
    pub async fn send(&self, packet: SphinxPacket, mix: &MixNode) -> Result<(), Box<dyn std::error::Error>>{
        let bytes = packet.to_bytes();

        let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
        stream.write_all(&bytes[..]).await?;
        Ok(())
    }
}


#[cfg(test)]
mod sending_a_sphinx_packet {
    use super::*;
    use sphinx::SphinxPacket;

    #[test]
    fn works() {
        // arrange
        let directory = Client::new();
        let message = "Hello, Sphinx!".as_bytes().to_vec();
        let mixes = directory.get_mixes();
        let destination = directory.get_destination();
        let delays = sphinx::header::delays::generate(2);
        let packet = SphinxPacket::new(message, &mixes, &destination, &delays).unwrap();
        let mix_client = MixClient::new();
        let first_hop = mixes.first().unwrap();

        // act
        mix_client.send(packet, first_hop);

        // assert
        // wtf are we supposed to assert here?
    }
}