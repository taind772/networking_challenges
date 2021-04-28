use std::{convert::TryInto, net::TcpStream};

use constants::BYTES_PER_U32;

use crate::{
    constants::STUDENT_ID,
    packet::{Packet, PacketType},
};

pub mod constants;
pub mod packet;

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.clone().to_owned().try_into().unwrap())
}

pub fn run(addr: &str, handle_calc: fn(&[u32]) -> u32) {
    let mut stream = TcpStream::connect(addr).unwrap();
    //Say hello to server
    println!("Sending hello");
    Packet::new(PacketType::PktHello, Some(STUDENT_ID.as_bytes()))
        .send(&mut stream)
        .expect("could not send hello to server");

    //Read request from server
    loop {
        let rev = Packet::read(&mut stream).unwrap();
        match rev.kind() {
            PacketType::PktHello => {
                println!("Server say hello!");
            }
            PacketType::PktCalc => {
                println!("Server requests to calc some math.");
                let data = rev
                    .data()
                    .unwrap()
                    .chunks_exact(BYTES_PER_U32)
                    .map(bytes_to_u32)
                    .collect::<Vec<_>>();
                let result = handle_calc(&data);
                Packet::new(PacketType::PktResult, Some(&result.to_le_bytes()))
                    .send(&mut stream)
                    .unwrap();
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
                let flag = rev.data().unwrap();
                println!("{}", flag.iter().map(|&v| v as char).collect::<String>());
                break;
            }
        }
    }
}
