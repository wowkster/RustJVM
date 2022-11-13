use std::io::prelude::*;
use std::{fs::File, io};

pub trait ByteParseable {
    fn parse_u1_as_bytes(&mut self) -> io::Result<[u8; 1]>;
    fn parse_u2_as_bytes(&mut self) -> io::Result<[u8; 2]>;
    fn parse_u4_as_bytes(&mut self) -> io::Result<[u8; 4]>;

    fn parse_u1(&mut self) -> io::Result<u8>;
    fn parse_u2(&mut self) -> io::Result<u16>;
    fn parse_u4(&mut self) -> io::Result<u32>;

    fn parse_u4_as_f32(&mut self) -> io::Result<f32>;
    fn parse_u4_as_i32(&mut self) -> io::Result<i32>;

    fn parse_u8_as_f64(&mut self) -> io::Result<f64>;
    fn parse_u8_as_i64(&mut self) -> io::Result<i64>;

    fn parse_utf8(&mut self, len: u16) -> io::Result<String>;
}

impl ByteParseable for File {
    fn parse_u1_as_bytes(&mut self) -> io::Result<[u8; 1]> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn parse_u2_as_bytes(&mut self) -> io::Result<[u8; 2]> {
        let mut buf = [0; 2];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn parse_u4_as_bytes(&mut self) -> io::Result<[u8; 4]> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn parse_u1(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;

        Ok(u8::from_be_bytes(buf))
    }

    fn parse_u2(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.read(&mut buf)?;

        Ok(u16::from_be_bytes(buf))
    }

    fn parse_u4(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;

        Ok(u32::from_be_bytes(buf))
    }

    fn parse_u4_as_f32(&mut self) -> io::Result<f32> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;

        Ok(f32::from_be_bytes(buf))
    }

    fn parse_u4_as_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;

        Ok(i32::from_be_bytes(buf))
    }

    fn parse_u8_as_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.read(&mut buf)?;

        Ok(f64::from_be_bytes(buf))
    }

    fn parse_u8_as_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.read(&mut buf)?;

        Ok(i64::from_be_bytes(buf))
    }

    fn parse_utf8(&mut self, len: u16) -> io::Result<String> {
        let mut buf = vec![0; len as usize];
        self.read(&mut buf)?;

        let str = String::from_utf8(buf).expect("Could not parse Utf-8");

        Ok(str)
    }
}
