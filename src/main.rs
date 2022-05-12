use artnet_protocol::{ArtCommand, Error as ArtError, Output, Poll, PortAddress};
use palette::{convert::IntoColor, Hsl, Srgb};
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
        let hue_mult = 11_f32;
        let white_mult = 0.7_f32;
        let output_fn = move |time: f32| {
            let hue = time * hue_mult;
            let color = Hsl::new(hue, 1_f32, 0.5_f32);
            let rgb_f: Srgb = color.into_color();
            let rgb_u: Srgb<u8> = rgb_f.into_format();
            let (r, g, b) = rgb_u.into_components();

            let white = ((((time / white_mult).sin() + 1_f32) / 2_f32) * 64_f32) as u8;

            vec![r, g, b, white]
            // vec![0, 0, 0, white]
        };

        let time_offset = |fun: Box<dyn Fn(f32) -> Vec<u8>>, offset: f32| {
            Box::new(move |time: f32| fun(time + offset))
        };
        let offset_mult = 20_f32 / hue_mult;

        Self {
            start_time: Instant::now(),
            socket: None,
            fixtures: vec![
                Fixture::from(RGBW {
                    address: 0,
                    output_fn: time_offset(Box::new(output_fn), 0_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 4,
                    output_fn: time_offset(Box::new(output_fn), 1_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 8,
                    output_fn: time_offset(Box::new(output_fn), 2_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 12,
                    output_fn: time_offset(Box::new(output_fn), 3_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 16,
                    output_fn: time_offset(Box::new(output_fn), 4_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 20,
                    output_fn: time_offset(Box::new(output_fn), 5_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 24,
                    output_fn: time_offset(Box::new(output_fn), 6_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 28,
                    output_fn: time_offset(Box::new(output_fn), 7_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 32,
                    output_fn: time_offset(Box::new(output_fn), 8_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 36,
                    output_fn: time_offset(Box::new(output_fn), 9_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 40,
                    output_fn: time_offset(Box::new(output_fn), 10_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 44,
                    output_fn: time_offset(Box::new(output_fn), 11_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 57,
                    output_fn: time_offset(Box::new(output_fn), 11_f32 * offset_mult),
                }),
                Fixture::from(RGBW {
                    address: 61,
                    output_fn: time_offset(Box::new(output_fn), 11_f32 * offset_mult),
                }),
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
            let mut data = vec![0; 65];
            let time = self.start_time.elapsed().as_secs_f32();

            for fixture in self.fixtures.iter_mut() {
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

            sleep(Duration::from_millis(5));
        }
    }
}
