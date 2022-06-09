use artnet_protocol::{ArtCommand, Error as ArtError, Output, Poll, PortAddress};
use std::io::Error as IoError;
use std::net::{ToSocketAddrs, UdpSocket};

use crate::output::OutputControl;

pub struct Artnet {
    socket: Option<UdpSocket>,
}

#[derive(Debug)]
pub enum ArtnetError {
    Io(IoError),
    Art(ArtError),
}

impl Artnet {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn artnet_poll(&mut self) -> Result<(), ArtnetError> {
        let socket = self.socket.as_ref().unwrap();

        let broadcast_addr = ("255.255.255.255", 6454)
            .to_socket_addrs()
            .map_err(|err| ArtnetError::Io(err))?
            .next()
            .unwrap();

        let buff = ArtCommand::Poll(Poll::default())
            .write_to_buffer()
            .map_err(|err| ArtnetError::Art(err))?;
        socket
            .send_to(&buff, &broadcast_addr)
            .map_err(|err| ArtnetError::Io(err))?;

        loop {
            let mut buffer = [0u8; 1024];
            let (length, addr) = socket
                .recv_from(&mut buffer)
                .map_err(|err| ArtnetError::Io(err))?;
            let command =
                ArtCommand::from_buffer(&buffer[..length]).map_err(|err| ArtnetError::Art(err))?;

            match command {
                ArtCommand::Poll(_poll) => {}
                ArtCommand::PollReply(reply) => {
                    let name: Vec<u8> = reply
                        .short_name
                        .into_iter()
                        .take_while(|v| *v != 0)
                        .collect();

                    if name != "CHAUVET".as_bytes().to_vec() {
                        continue;
                    }

                    println!("Node {:?}", reply);
                    println!("Addr {:?}", addr);
                }
                _ => {}
            }
        }
    }
}

impl OutputControl for Artnet {
    type Error = ArtnetError;

    fn setup(&mut self) -> Result<(), Self::Error> {
        let socket = UdpSocket::bind(("0.0.0.0", 6454)).map_err(|err| ArtnetError::Io(err))?;

        socket
            .set_broadcast(true)
            .map_err(|err| ArtnetError::Io(err))?;

        self.socket = Some(socket);

        Ok(())
    }

    fn output(&mut self, dmx: &mut Vec<u8>) -> Result<(), Self::Error> {
        let addr = "192.168.60.99:6454";
        let socket = self.socket.as_ref().unwrap();

        let port_address: PortAddress = 0.into();

        let command = ArtCommand::Output(Output {
            data: dmx.clone().into(),
            port_address,
            ..Output::default()
        });

        let bytes = command
            .write_to_buffer()
            .map_err(|err| ArtnetError::Art(err))?;

        // println!("Output: {:?}", bytes);

        socket
            .send_to(&bytes, &addr)
            .map_err(|err| ArtnetError::Io(err))?;

        Ok(())
    }
}
