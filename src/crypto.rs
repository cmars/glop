extern crate byteorder;
extern crate sodiumoxide;
extern crate tokio_core;

use std;
use self::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use self::sodiumoxide::crypto::{box_, secretbox};
use self::tokio_core::io::{Codec, EasyBuf};

pub struct SecretBoxCodec<T: Codec> {
    codec: T,
    key: secretbox::Key,
}

impl<T: Codec> SecretBoxCodec<T> {
    pub fn new(codec: T, key: secretbox::Key) -> SecretBoxCodec<T> {
        SecretBoxCodec {
            codec: codec,
            key: key,
        }
    }
}

impl<T: Codec> Codec for SecretBoxCodec<T> {
    type Out = T::Out;
    type In = T::In;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<T::In>> {
        if buf.len() < secretbox::NONCEBYTES + 4 {
            return Ok(None);
        }
        let h_buf = buf.drain_to(secretbox::NONCEBYTES + 4);
        let mut h = h_buf.as_slice();

        let nonce = secretbox::Nonce::from_slice(&h[0..secretbox::NONCEBYTES]).unwrap();
        h = &h[secretbox::NONCEBYTES..];
        let c_len = BigEndian::read_u32(&h[0..4]) as usize;

        if buf.len() < c_len {
            return Ok(None);
        }
        let c_buf = buf.drain_to(c_len);
        match secretbox::open(c_buf.as_slice(), &nonce, &self.key) {
            Ok(p) => {
                debug!("open ok");
                self.codec.decode(&mut EasyBuf::from(p))
            }
            Err(()) => {
                debug!("open failed, invalid ciphertext");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "message invalid"))
            }
        }
    }

    fn encode(&mut self, msg: T::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        let nonce = secretbox::gen_nonce();
        buf.extend_from_slice(&nonce.0);

        let mut p = vec![];
        self.codec.encode(msg, &mut p)?;
        let mut c = secretbox::seal(&p, &nonce, &self.key);

        let mut l = vec![];
        l.write_u32::<BigEndian>(c.len() as u32)?;

        buf.append(&mut l);
        buf.append(&mut c);
        Ok(())
    }
}

pub struct BoxCodec<T: Codec> {
    codec: T,
    pubkey: box_::PublicKey,
    seckey: box_::SecretKey,
}

impl<T: Codec> BoxCodec<T> {
    pub fn new(codec: T, pubkey: box_::PublicKey, seckey: box_::SecretKey) -> BoxCodec<T> {
        BoxCodec {
            codec: codec,
            pubkey: pubkey,
            seckey: seckey,
        }
    }
}

impl<T: Codec> Codec for BoxCodec<T> {
    type Out = T::Out;
    type In = T::In;

    fn decode(&mut self, buf: &mut EasyBuf) -> std::io::Result<Option<T::In>> {
        if buf.len() < box_::NONCEBYTES + 4 {
            return Ok(None);
        }
        let h_buf = buf.drain_to(box_::NONCEBYTES + 4);
        let mut h = h_buf.as_slice();

        let nonce = box_::Nonce::from_slice(&h[0..box_::NONCEBYTES]).unwrap();
        h = &h[box_::NONCEBYTES..];
        let c_len = BigEndian::read_u32(&h[0..4]) as usize;

        if buf.len() < c_len {
            return Ok(None);
        }
        let c_buf = buf.drain_to(c_len);
        match box_::open(c_buf.as_slice(), &nonce, &self.pubkey, &self.seckey) {
            Ok(p) => {
                debug!("open ok");
                self.codec.decode(&mut EasyBuf::from(p))
            }
            Err(()) => {
                debug!("open failed, invalid ciphertext");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "message invalid"))
            }
        }
    }

    fn encode(&mut self, msg: T::Out, buf: &mut Vec<u8>) -> std::io::Result<()> {
        let nonce = box_::gen_nonce();
        buf.extend_from_slice(&nonce.0);

        let mut p = vec![];
        self.codec.encode(msg, &mut p)?;
        let mut c = box_::seal(&p, &nonce, &self.pubkey, &self.seckey);

        let mut l = vec![];
        l.write_u32::<BigEndian>(c.len() as u32)?;

        buf.append(&mut l);
        buf.append(&mut c);
        Ok(())
    }
}
