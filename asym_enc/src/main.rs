extern crate clap;
extern crate openssl;

use clap::{Arg, App};
use openssl::rand;
use openssl::rsa::{self, Rsa};
use openssl::symm::{self, Cipher};

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn parse_and_crypt() -> Result<(), String> {
    let matches = App::new("asym_enc")
                      .arg(Arg::with_name("mode")
                               .short("m")
                               .value_name("MODE")
                               .help("enc/dec (encryption/decryption mode)")
                               .takes_value(true))
                      .arg(Arg::with_name("in")
                               .short("i")
                               .value_name("FILE")
                               .help("input file to encrypt/decrypt.")
                               .takes_value(true))
                      .arg(Arg::with_name("out")
                               .short("o")
                               .value_name("FILE")
                               .help("encrypted/decrypted file output.")
                               .takes_value(true))
                      .arg(Arg::with_name("pem")
                               .short("p")
                               .value_name("KEY")
                               .help("public/private key in PEM format.")
                               .takes_value(true))
                      .arg(Arg::with_name("key")
                               .short("k")
                               .value_name("KEY")
                               .help("symmetric key input/output.")
                               .takes_value(true))
                      .get_matches();

    let is_encrypt = match matches.value_of("mode") {
        Some("enc") => true,
        Some("dec") => false,
        _ => return Err(format!("Expected one of enc/dec for mode!")),
    };

    let mut bytes = vec![];
    match matches.value_of("in") {    // large files should probably be buffered?
        Some(f) => File::open(f).map(BufReader::new)
                        .map_err(|e| format!("Cannot open the input file! ({})", e))?
                        .read_to_end(&mut bytes).map_err(|e| format!("Cannot read input file! ({})", e))?,
        None => return Err(format!("Input file required!")),
    };

    let mut key = vec![];
    match matches.value_of("pem") {
        Some(f) => File::open(f)
                        .map_err(|e| format!("Cannot open the PEM file! ({})", e))?
                        .read_to_end(&mut key).map_err(|e| format!("Cannot read from PEM file! ({})", e))?,
        None => return Err(format!("PEM key required!")),
    };

    let pem = if is_encrypt {
        Rsa::public_key_from_pem(&key)
    } else {
        Rsa::private_key_from_pem(&key)
    }.map_err(|e| format!("Cannot deserialize PEM key! ({})", e))?;

    let mut out_file = match matches.value_of("out") {
        Some(f) => BufWriter::new(File::create(f).map_err(|e| format!("Cannot create output file! ({})", e))?),
        None => return Err(format!("Output file required!")),
    };

    let mut key_file = match matches.value_of("key") {
        Some(f) => if is_encrypt {
            File::create(f).map_err(|e| format!("Cannot create output AES key! ({})", e))?
        } else {
            File::open(f).map_err(|e| format!("Cannot open AES key file! ({})", e))?
        },
        None => return Err(format!("Output AES key required!")),
    };

    let cipher = Cipher::aes_256_cbc();     // user input?
    let init_vec = vec![0; cipher.iv_len().unwrap()];

    let out_bytes = if is_encrypt {
        let mut key_file = BufWriter::new(key_file);
        let mut rand_key = vec![0; cipher.key_len()];     // generate random key
        rand::rand_bytes(&mut rand_key).map_err(|e| format!("Cannot generate random key for AES ({})", e))?;
        let out_bytes = symm::encrypt(cipher, &rand_key, Some(&init_vec), &bytes)     // encrypt with AES
                             .map_err(|e| format!("Cannot encrypt block! ({})", e))?;

        let mut encrypted_key = vec![0; pem.size()];
        pem.public_encrypt(&rand_key, &mut encrypted_key, rsa::PKCS1_PADDING)     // encrypt with public key
           .map_err(|e| format!("Cannot encrypt using the public key! ({})", e))?;
        key_file.write_all(&encrypted_key).map_err(|e| format!("Cannot write the AES key to file! ({})", e))?;
        out_bytes
    } else {
        let mut key = vec![];
        key_file.read_to_end(&mut key).map_err(|e| format!("Cannot read AES key! ({})", e))?;
        let mut decrypted_key = vec![0; pem.size()];
        let size = pem.private_decrypt(&key, &mut decrypted_key, rsa::PKCS1_PADDING)    // decrypt AES key with private key
                      .map_err(|e| format!("Cannot decrypt using the private key! ({})", e))?;

        symm::decrypt(cipher, &decrypted_key[..size], Some(&init_vec), &bytes)    // decrypt with the decrypted AES key
             .map_err(|e| format!("Cannot decrypt block! ({})", e))?
    };

    out_file.write_all(&out_bytes).map_err(|e| format!("Cannot write to output file! ({})", e))?;
    Ok(())
}

fn main() {
    openssl::init();
    if let Err(e) = parse_and_crypt() {
        println!("{}", e);
    }
}
