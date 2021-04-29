use std::{convert::TryInto, net::TcpStream};

use constants::BYTES_PER_U32;

use crate::{
    packet::{Packet, PacketType},
};

pub mod constants;
pub mod packet;
pub mod solve;

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(
        bytes
            .clone()
            .to_owned()
            .try_into()
            .expect(&format!("Could not parse bytes: {:?}", bytes)),
    )
}

pub fn run(addr: &str, student_id: &str, handle_calc: fn(&[u32]) -> u32) {
    let mut stream =
        TcpStream::connect(addr).expect(&format!("Could not connect to TcpStream at {}", addr));
    //Say hello to server
    println!("Sending hello");
    Packet::new(PacketType::PktHello, Some(student_id.as_bytes()))
        .send(&mut stream)
        .expect("Could not send hello to server");

    //Read request from server
    loop {
        let rev = Packet::read(&mut stream).expect("Could not read the receive packet from stream");
        match rev.kind() {
            PacketType::PktHello => {
                println!("Server say hello!");
            }
            PacketType::PktCalc => {
                println!("Server requests to calc some math.");
                if let Some(data) = rev.data() {
                    let data = data
                        .chunks_exact(BYTES_PER_U32)
                        .map(bytes_to_u32)
                        .collect::<Vec<_>>();
                    let result = handle_calc(&data);
                    Packet::new(PacketType::PktResult, Some(&result.to_le_bytes()))
                        .send(&mut stream)
                        .expect("Could not send result back to server");
                } else {
                    println!("Calc-packet's data is empty.")
                }
            }
            PacketType::PktResult => {
                panic!("Something went wrong! -- Server should not response a PktResult at all!");
            }
            PacketType::PktBye => {
                println!("Server sent bye! >> Some broken logics happen");
                break;
            }
            PacketType::PktFlag => {
                println!("Ok");
                if let Some(flag) = rev.data() {
                    println!(
                        "{}",
                        flag.iter().map(|&v| char::from(v)).collect::<String>()
                    );
                    break;
                }
            }
        }
    }
}
