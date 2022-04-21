use artnet_protocol::{ArtCommand, Error as ArtError, Output, Poll};
use std::io::Error as IoError;
use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    let mut server = ArtServer::new();
    server.connect().unwrap();
    server.start().unwrap();
}

pub struct ArtServer {
    socket: Option<UdpSocket>,
}

#[derive(Debug)]
pub enum ArtServerError {
    Io(IoError),
    Art(ArtError),
}

impl ArtServer {
    pub fn new() -> Self {
        Self { socket: None }
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
            let (length, addr) = socket
                .recv_from(&mut buffer)
                .map_err(|err| ArtServerError::Io(err))?;
            let command = ArtCommand::from_buffer(&buffer[..length])
                .map_err(|err| ArtServerError::Art(err))?;

            println!("Received {:?}", command);

            match command {
                ArtCommand::Poll(_poll) => {
                    // this will most likely be our own poll request, as this is broadcast to all devices on the network
                }
                ArtCommand::PollReply(_reply) => {
                    // this is an ArtNet node on the network. We can send commands to it like this:
                    let command = ArtCommand::Output(Output {
                        data: vec![1, 2, 3, 4, 5].into(),
                        ..Output::default()
                    });
                    let bytes = command
                        .write_to_buffer()
                        .map_err(|err| ArtServerError::Art(err))?;
                    socket
                        .send_to(&bytes, &addr)
                        .map_err(|err| ArtServerError::Io(err))?;
                }
                _ => {}
            }
        }
    }
}
