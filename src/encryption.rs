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

fn aes_encrypt(
    key: &[u8],
    // iv: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = Cipher::aes_256_ecb();
    let ciphertext = encrypt(cipher, key, None, plaintext)?;
    Ok(ciphertext)
}

fn aes_decrypt(
    key: &[u8],
    // iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = Cipher::aes_256_ecb();
    let plaintext = decrypt(cipher, key, None, ciphertext)?;
    Ok(plaintext)
}

#[allow(deprecated)]
pub fn handshake(mut stream: TcpStream, rsa: Rsa<Private>) -> ETcp {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    stream.write_all(&rsa.public_key_to_pem().unwrap()).unwrap();

    stream.flush().unwrap();

    let mut aes_encoded = String::new();

    reader.read_line(&mut aes_encoded).unwrap();

    let encrypted_data = base64::decode(aes_encoded.trim()).expect("Failed to decode base64");

    let mut aes_decrypted = vec![0; 256];

    let aes_len = rsa
        .private_decrypt(
            &encrypted_data,
            &mut aes_decrypted,
            openssl::rsa::Padding::PKCS1,
        )
        .expect("Decryption failed");

    println!("len: {}", aes_len);

    aes_decrypted.truncate(aes_len);

    println!(
        "Decrypted AES key ({} bytes): {:?}",
        aes_decrypted.len(),
        aes_decrypted
    );

    ETcp {
        stream,
        aes: aes_decrypted,
        reader,
    }
}

pub struct ETcp {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    aes: Vec<u8>,
}

impl ETcp {
    #[allow(deprecated)]
    pub fn send<T: Display>(&mut self, d: T) {
        let a = aes_encrypt(&self.aes, d.to_string().as_bytes()).unwrap();

        self.stream
            .write_all((base64::encode(a) + "\n").as_bytes())
            .unwrap();

        self.stream.flush().unwrap();
    }

    #[allow(deprecated)]
    pub fn read(&mut self) -> Option<String> {
        let mut v_encoded = String::new();

        self.reader.read_line(&mut v_encoded).unwrap();

        let encrypted_data = base64::decode(v_encoded.trim()).expect("Failed to decode base64");

        Some(String::from_utf8(aes_decrypt(&self.aes, &encrypted_data).unwrap()).unwrap())
    }

    pub fn close(&self) {
        self.stream
            .shutdown(std::net::Shutdown::Both)
            .expect("shutdown call failed");
    }

    pub fn clone(&mut self) -> Self {
        let stream = self.stream.try_clone().unwrap();
        ETcp {
            reader: BufReader::new(stream.try_clone().unwrap()),
            stream,
            aes: self.aes.clone(),
        }
    }
}
