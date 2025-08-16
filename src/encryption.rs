use std::{
    fmt::Display,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use openssl::{
    pkey::Private,
    rsa::Rsa,
    symm::{decrypt, encrypt, Cipher},
};

use crate::response::Error;

fn aes_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = Cipher::aes_256_ecb();
    encrypt(cipher, key, None, plaintext)
        .map_err(|e| Error::EncryptionError(format!("Failed to encrypt: {}", e)))
}

fn aes_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = Cipher::aes_256_ecb();
    decrypt(cipher, key, None, ciphertext)
        .map_err(|e| Error::EncryptionError(format!("Failed to decrypt: {}", e)))
}

#[allow(deprecated)]
pub fn handshake(mut stream: TcpStream, rsa: Rsa<Private>) -> Result<ETcp, Error> {
    let stream_clone = stream
        .try_clone()
        .map_err(|e| Error::NetworkError(format!("Failed to clone stream: {}", e)))?;
    let mut reader = BufReader::new(stream_clone);

    let public_key = rsa
        .public_key_to_pem()
        .map_err(|e| Error::EncryptionError(format!("Failed to get public key: {}", e)))?;
    stream
        .write_all(&public_key)
        .map_err(|e| Error::NetworkError(format!("Failed to write public key: {}", e)))?;

    stream
        .flush()
        .map_err(|e| Error::NetworkError(format!("Failed to flush stream: {}", e)))?;

    let mut aes_encoded = String::new();
    reader
        .read_line(&mut aes_encoded)
        .map_err(|e| Error::NetworkError(format!("Failed to read AES key: {}", e)))?;

    let encrypted_data = base64::decode(aes_encoded.trim())
        .map_err(|e| Error::EncryptionError(format!("Failed to decode base64: {}", e)))?;

    let mut aes_decrypted = vec![0; 256];
    let aes_len = rsa
        .private_decrypt(
            &encrypted_data,
            &mut aes_decrypted,
            openssl::rsa::Padding::PKCS1,
        )
        .map_err(|e| Error::EncryptionError(format!("Failed to decrypt AES key: {}", e)))?;

    println!("len: {}", aes_len);
    aes_decrypted.truncate(aes_len);

    println!(
        "Decrypted AES key ({} bytes): {:?}",
        aes_decrypted.len(),
        aes_decrypted
    );

    Ok(ETcp {
        stream,
        aes: aes_decrypted,
        reader,
    })
}

pub struct ETcp {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    aes: Vec<u8>,
}

impl ETcp {
    #[allow(deprecated)]
    pub fn send<T: Display>(&mut self, d: T) -> Result<(), Error> {
        let encrypted = aes_encrypt(&self.aes, d.to_string().as_bytes())?;
        let encoded = base64::encode(encrypted) + "\n";

        self.stream
            .write_all(encoded.as_bytes())
            .map_err(|e| Error::NetworkError(format!("Failed to write to stream: {}", e)))?;

        self.stream
            .flush()
            .map_err(|e| Error::NetworkError(format!("Failed to flush stream: {}", e)))
    }

    #[allow(deprecated)]
    pub fn read(&mut self) -> Result<Option<String>, Error> {
        let mut v_encoded = String::new();

        if self
            .reader
            .read_line(&mut v_encoded)
            .map_err(|e| Error::NetworkError(format!("Failed to read line: {}", e)))?
            == 0
        {
            return Ok(None);
        }

        let encrypted_data = base64::decode(v_encoded.trim())
            .map_err(|e| Error::EncryptionError(format!("Failed to decode base64: {}", e)))?;

        let decrypted = aes_decrypt(&self.aes, &encrypted_data)?;
        let text = String::from_utf8(decrypted)
            .map_err(|e| Error::EncryptionError(format!("Failed to decode UTF-8: {}", e)))?;

        Ok(Some(text))
    }

    pub fn close(&self) -> Result<(), Error> {
        self.stream
            .shutdown(std::net::Shutdown::Both)
            .map_err(|e| Error::NetworkError(format!("Failed to shutdown stream: {}", e)))
    }

    pub fn try_clone(&mut self) -> Result<Self, Error> {
        let stream = self
            .stream
            .try_clone()
            .map_err(|e| Error::NetworkError(format!("Failed to clone stream: {}", e)))?;
        let stream_clone = stream
            .try_clone()
            .map_err(|e| Error::NetworkError(format!("Failed to clone stream: {}", e)))?;

        Ok(ETcp {
            reader: BufReader::new(stream_clone),
            stream,
            aes: self.aes.clone(),
        })
    }
}

impl Clone for ETcp {
    fn clone(&self) -> Self {
        let stream = self
            .stream
            .try_clone()
            .map_err(|e| Error::NetworkError(format!("Failed to clone stream: {}", e)))
            .unwrap();
        let stream_clone = stream
            .try_clone()
            .map_err(|e| Error::NetworkError(format!("Failed to clone stream: {}", e)))
            .unwrap();

        ETcp {
            reader: BufReader::new(stream_clone),
            stream,
            aes: self.aes.clone(),
        }
    }
}
