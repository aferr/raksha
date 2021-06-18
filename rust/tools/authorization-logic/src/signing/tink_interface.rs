use crate::ast::*;
use std::fs::File;
use tink_core::{keyset, TinkError};
use std::io::{Read,Write, Error};

// The specific choice of how serialization is done is encapsulated because it
// is likely to change. Binaries are brittle way of serializing objects, but
// this was reasonable to get running and is easy to change later.
fn serialize_claim(claim: &Vec<AstAssertion>)-> Vec<u8> {
    bincode::serialize(&claim).unwrap()
}

fn deserialize_claim(claim_bin: Vec<u8>)-> Vec<AstAssertion> {
    bincode::deserialize(&claim_bin[..]).unwrap()
}

pub fn serialize_to_file(claim: &Vec<AstAssertion>,
                         filename: &String) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write_all(&serialize_claim(claim))?;
    Ok(())
}

pub fn deserialize_from_file(filename: &String)
    -> Result<Vec<AstAssertion>, Error> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(deserialize_claim(contents))
}

// At present, this function is only used in tests, so a warning will be given
// that this is not used. In the future, a script for generating keys might
// also use this outside of tests.
pub fn store_new_keypair_cleartext(pub_key_file: &String,
        priv_key_file: &String) {
    tink_signature::init();

    // create new signing keypair
    let key_handle = tink_core::keyset::Handle::new(
        &tink_signature::ecdsa_p256_key_template()).unwrap();

    // write the new private key
    let prv_file = File::create(priv_key_file).unwrap();
    // See also tink_core::keyset::BinWriter/BinReader
    let mut prv_writer = keyset::JsonWriter::new(prv_file);
    // This stores the private key in plaintext, If possible, it would be better
    // to store the private key using a google cloud or aws key manager.
    keyset::insecure::write(&key_handle, &mut prv_writer).unwrap();

    // write the new public key
    let pub_key_handle = key_handle.public().unwrap();
    let pub_file = File::create(pub_key_file).unwrap();
    let mut pub_writer = keyset::JsonWriter::new(pub_file);
    keyset::insecure::write(&pub_key_handle, &mut pub_writer).unwrap();
}

pub fn sign_claim(priv_key_file: &String, signature_file: &String, 
        claim: &Vec<AstAssertion>) {
    // read private key from file
    let prv_file = File::open(priv_key_file).unwrap();
    let mut json_reader = keyset::JsonReader::new(prv_file);
    let priv_kh = keyset::insecure::read(&mut json_reader).unwrap();

    // write signature to file
    // let bytes = b"bytes to test signing";
    let bytes = serialize_claim(&claim);
    let signer = tink_signature::new_signer(&priv_kh).unwrap();
    let signature = signer.sign(&bytes).unwrap();

    let mut sig_file = File::create(signature_file).unwrap();
    sig_file.write(&signature).unwrap();
}

pub fn verify_claim(pub_key_file: &String,
        signature_file: &String, claim: &Vec<AstAssertion>) 
            -> Result<(), TinkError> {
    // read pub key from file
    let pub_file = File::open(pub_key_file).unwrap();
    let mut json_reader = keyset::JsonReader::new(pub_file);
    let pub_kh = keyset::insecure::read(&mut json_reader).unwrap();

    // let plaintext = b"bytes to test signing";
    let plaintext = serialize_claim(&claim);
    let v = tink_signature::new_verifier(&pub_kh).unwrap();
    let mut signature_buf = Vec::new();
    File::open(&signature_file).unwrap()
        .read_to_end(&mut signature_buf).unwrap();
    v.verify(&signature_buf, &plaintext)
}

