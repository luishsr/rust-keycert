extern crate openssl;

use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::x509::{X509, X509NameBuilder, X509Extension};
use openssl::x509::extension::{SubjectAlternativeName, KeyUsage, ExtendedKeyUsage};
use openssl::pkey::PKey;
use std::fs::File;
use std::io::Write;

fn main() {
    // Generate an ECDSA private key using the prime256v1 curve.
    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
    let eckey = EcKey::generate(&group).unwrap();
    let pkey = PKey::from_ec_key(eckey).unwrap();

    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();

    // Set the certificate's serial number
    builder.set_serial_number(&openssl::bn::BigNum::from_u32(1).unwrap().to_asn1_integer().unwrap()).unwrap();

    // Set issuer and subject name
    let mut name = X509NameBuilder::new().unwrap();
    name.append_entry_by_text("CN", "localhost").unwrap();
    let name = name.build();
    builder.set_issuer_name(&name).unwrap();
    builder.set_subject_name(&name).unwrap();
    builder.set_pubkey(&pkey).unwrap();

    // Validity
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365).unwrap();
    builder.set_not_before(&not_before).unwrap();
    builder.set_not_after(&not_after).unwrap();

    // Subject Alternative Name
    let mut san = SubjectAlternativeName::new();
    san.dns("localhost");
    let extension = san.build(&builder.x509v3_context(None, None)).unwrap();
    builder.append_extension(extension).unwrap();

    // Key Usage
    let key_usage = KeyUsage::new().digital_signature().build().unwrap();
    builder.append_extension(key_usage).unwrap();

    // Extended Key Usage
    let server_auth = ExtendedKeyUsage::new().server_auth().build().unwrap();
    builder.append_extension(server_auth).unwrap();

    // Sign the certificate with the private key
    builder.sign(&pkey, openssl::hash::MessageDigest::sha256()).unwrap();
    let certificate = builder.build();

    // Save the private key and certificate in PEM format
    let mut privkey_file = File::create("localhost.key").unwrap();
    let mut cert_file = File::create("localhost.crt").unwrap();

    privkey_file.write_all(pkey.private_key_to_pem_pkcs8().unwrap().as_ref()).unwrap();
    cert_file.write_all(certificate.to_pem().unwrap().as_ref()).unwrap();

    println!("Private key saved to: localhost.key");
    println!("Certificate saved to: localhost.crt");
}
