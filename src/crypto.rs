extern crate bytes;
extern crate byteorder;
extern crate sodiumoxide;
extern crate tokio_core;
extern crate tokio_io;

use std;
use self::bytes::{BufMut, BytesMut};
use self::byteorder::{BigEndian, ByteOrder};
use self::sodiumoxide::crypto::secretbox;
use self::tokio_io::codec::{Decoder, Encoder};

pub struct SecretBoxCodec<T: Decoder + Encoder> {
    codec: T,
    key: secretbox::Key,
}

impl<T: Decoder + Encoder> SecretBoxCodec<T>
    where T: Decoder
{
    pub fn new(codec: T, key: secretbox::Key) -> SecretBoxCodec<T> {
        SecretBoxCodec {
            codec: codec,
            key: key,
        }
    }
}

impl<T: Decoder> Decoder for SecretBoxCodec<T>
    where T: Encoder, T: Decoder<Error = std::io::Error>
{
    type Item = <T as Decoder>::Item;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> std::io::Result<Option<<T as Decoder>::Item>> {
        if buf.len() < secretbox::NONCEBYTES + 4 {
            return Ok(None);
        }
        let nonce_buf = buf.split_to(secretbox::NONCEBYTES);
        let nonce = secretbox::Nonce::from_slice(&nonce_buf[..]).unwrap();

        let c_len_buf = buf.split_to(4);
        let c_len = BigEndian::read_u32(&c_len_buf[0..4]) as usize;

        if buf.len() < c_len {
            return Ok(None);
        }
        match secretbox::open(&buf[..c_len], &nonce, &self.key) {
            Ok(p) => {
                debug!("open ok");
                self.codec.decode(&mut BytesMut::from(p))
            }
            Err(()) => {
                debug!("open failed, invalid ciphertext");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "message invalid"))
            }
        }
    }
}

impl<T: Encoder> Encoder for SecretBoxCodec<T>
    where T: Decoder, T: Encoder<Error = std::io::Error>
{
    type Item = <T as Encoder>::Item;
    type Error = std::io::Error;

    fn encode(&mut self, msg: <T as Encoder>::Item, buf: &mut BytesMut) -> std::io::Result<()> {
        let nonce = secretbox::gen_nonce();
        buf.extend(&nonce.0);

        let mut p = BytesMut::with_capacity(0);
        self.codec.encode(msg, &mut p)?;
        let c = secretbox::seal(&p[..], &nonce, &self.key);
        buf.reserve(4);
        buf.put_u32::<BigEndian>(c.len() as u32);
        buf.extend(&c[..]);
        Ok(())
    }
}
