use artnet_protocol::{ArtCommand, Error as ArtError, Output, Poll, PortAddress};
use std::io::Error as IoError;
use std::net::{ToSocketAddrs, UdpSocket};
use std::thread::sleep;
use std::time::{Duration, Instant};

use vibenet::{
    fixture::FixtureControl,
    fixtures::{Fixture, RGBW},
};

fn main() {
    let mut server = ArtServer::new();
    server.connect().unwrap();
    server.artnet_output().unwrap();
}

pub struct ArtServer {
    start_time: Instant,
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
            start_time: Instant::now(),
            socket: None,
            fixtures: vec![
                Fixture::from(RGBW { address: 0 }),
                Fixture::from(RGBW { address: 4 }),
                Fixture::from(RGBW { address: 8 }),
                Fixture::from(RGBW { address: 12 }),
                Fixture::from(RGBW { address: 16 }),
                Fixture::from(RGBW { address: 20 }),
                Fixture::from(RGBW { address: 24 }),
                Fixture::from(RGBW { address: 28 }),
                Fixture::from(RGBW { address: 32 }),
                Fixture::from(RGBW { address: 36 }),
                Fixture::from(RGBW { address: 40 }),
                Fixture::from(RGBW { address: 44 }),
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

    pub fn artnet_poll(&mut self) -> Result<(), ArtServerError> {
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
            let (length, addr) = socket
                .recv_from(&mut buffer)
                .map_err(|err| ArtServerError::Io(err))?;
            let command = ArtCommand::from_buffer(&buffer[..length])
                .map_err(|err| ArtServerError::Art(err))?;

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

    pub fn artnet_output(&mut self) -> Result<(), ArtServerError> {
        let addr = "192.168.60.99:6454";
        let socket = self.socket.as_ref().unwrap();

        loop {
            let mut data = vec![0; 48];
            let time = self.start_time.elapsed().as_secs_f32();

            for mut fixture in self.fixtures.clone() {
                fixture.write_output(&mut data, time);
            }

            let port_address: PortAddress = 0.into();

            let command = ArtCommand::Output(Output {
                data: data.into(),
                port_address,
                ..Output::default()
            });
            let bytes = command
                .write_to_buffer()
                .map_err(|err| ArtServerError::Art(err))?;

            println!("Output: {:?}", bytes);

            socket
                .send_to(&bytes, &addr)
                .map_err(|err| ArtServerError::Io(err))?;

            sleep(Duration::from_millis(30));
        }
    }
}
