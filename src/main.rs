#![allow(unused_imports)]
use std::{
    io::{Cursor, Read, Seek, Write},
    net::TcpListener,
};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0_u8; 4];
                _stream.read_exact(buf.as_mut_slice()).unwrap();
                let mut rdr = Cursor::new(buf);
                let len = rdr.read_u32().await.unwrap();
                let mut msg = vec![0u8; len.try_into().unwrap()];
                _stream.read_exact(msg.as_mut_slice()).unwrap();
                let mut rdr = Cursor::new(msg);
                rdr.read_u16().await.unwrap();
                rdr.read_u16().await.unwrap();
                let correlation_id = rdr.read_u32().await.unwrap();

                _stream.write_all([0, 0, 0, 4].as_slice()).unwrap();
                _stream.write_all(&correlation_id.to_be_bytes()).unwrap();
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
