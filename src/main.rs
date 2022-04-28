use artnet_protocol::{ArtCommand, Error as ArtError, Output, Poll};
use std::boxed::Box;
use std::io::Error as IoError;
use std::net::{ToSocketAddrs, UdpSocket};

use vibenet::{
    fixture::FixtureControl,
    fixtures::{Fixture, RGBW},
};

fn main() {
    let mut server = ArtServer::new();
    server.connect().unwrap();
    server.start().unwrap();
}

pub struct ArtServer {
    socket: Option<UdpSocket>,
    fixtures: Vec<Fixture>,
}

#[derive(Debug)]
pub enum ArtServerError {
    Io(IoError),
    Art(ArtError),
}

impl ArtServer {
    pub fn new() -> Self {
        Self {
            socket: None,
            fixtures: vec![
                Fixture::from(RGBW { address: 0 }),
                Fixture::from(RGBW { address: 4 }),
            ],
        }
    }

    pub fn connect(&mut self) -> Result<(), ArtServerError> {
        let socket = UdpSocket::bind(("0.0.0.0", 6454)).map_err(|err| ArtServerError::Io(err))?;

        socket
            .set_broadcast(true)
            .map_err(|err| ArtServerError::Io(err))?;

        self.socket = Some(socket);

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), ArtServerError> {
        let socket = self.socket.as_ref().unwrap();

        let broadcast_addr = ("255.255.255.255", 6454)
            .to_socket_addrs()
            .map_err(|err| ArtServerError::Io(err))?
            .next()
            .unwrap();

        let buff = ArtCommand::Poll(Poll::default())
            .write_to_buffer()
            .map_err(|err| ArtServerError::Art(err))?;
        socket
            .send_to(&buff, &broadcast_addr)
            .map_err(|err| ArtServerError::Io(err))?;

        loop {
            let mut buffer = [0u8; 1024];
            let (length, _addr) = socket
                .recv_from(&mut buffer)
                .map_err(|err| ArtServerError::Io(err))?;
            let command = ArtCommand::from_buffer(&buffer[..length])
                .map_err(|err| ArtServerError::Art(err))?;

            println!("Received {:?}", command);

            match command {
                ArtCommand::Poll(_poll) => {}
                ArtCommand::PollReply(_reply) => {
                    let mut data = vec![0; 512];
                    let time = 1_f32;

                    for mut fixture in self.fixtures.clone() {
                        fixture.write_output(&mut data, time);
                    }

                    let command = ArtCommand::Output(Output {
                        data: data.into(),
                        ..Output::default()
                    });
                    let bytes = command
                        .write_to_buffer()
                        .map_err(|err| ArtServerError::Art(err))?;

                    println!("Output: {:?}", bytes);

                    /*
                    socket
                        .send_to(&bytes, &addr)
                        .map_err(|err| ArtServerError::Io(err))?;
                    */
                }
                _ => {}
            }
        }
    }
}
