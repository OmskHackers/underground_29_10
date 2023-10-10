use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::Error;
use std::str;

pub struct Session {
    buf: [u8; 1024],
    stream : TcpStream
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Session {
            buf: [0u8; 1024],
            stream: stream 
        }
    }
    pub async fn write(&mut self, data: String) -> Option<Error> {
        if let Err(e) = self.stream.write_all(data.as_bytes()).await {
            return Some(e);
        }
        None
    }
    pub async fn read(&mut self) -> Result<String, Error> {
        match self.stream.read(&mut self.buf).await {
            Ok(n) if n == 0 => Ok("".to_string()),
            Ok(n) => {
                let payload = str::from_utf8(&self.buf[0..n-1]).unwrap();
                Ok(payload.to_string())
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}