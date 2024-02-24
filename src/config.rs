use std::{
    fs,
    io::{self, Read},
};

use anyhow::Context;

use rustls::Certificate;

pub fn load() -> anyhow::Result<quinn::ServerConfig> {
    // Read the PEM certificate chain
    let chain = fs::File::open("cert/localhost.crt").context("failed to open cert file")?;
    let mut chain = io::BufReader::new(chain);

    let chain: Vec<Certificate> = rustls_pemfile::certs(&mut chain)?
        .into_iter()
        .map(Certificate)
        .collect();

    anyhow::ensure!(!chain.is_empty(), "could not find certificate");

    // Read the PEM private key
    let mut keys = fs::File::open("cert/localhost.key").context("failed to open key file")?;

    // Read the keys into a Vec so we can parse it twice.
    let mut buf = Vec::new();
    keys.read_to_end(&mut buf)?;

    // Try to parse a PKCS#8 key
    // -----BEGIN PRIVATE KEY-----
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut io::Cursor::new(&buf))?;

    // Try again but with EC keys this time
    // -----BEGIN EC PRIVATE KEY-----
    if keys.is_empty() {
        keys = rustls_pemfile::ec_private_keys(&mut io::Cursor::new(&buf))?
    };

    anyhow::ensure!(!keys.is_empty(), "could not find private key");
    anyhow::ensure!(keys.len() < 2, "expected a single key");

    //let certs = certs.into_iter().map(rustls::Certificate).collect();
    let key = rustls::PrivateKey(keys.remove(0));

    // Standard Quinn setup
    let mut tls_config = rustls::ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(chain, key)?;

    tls_config.max_early_data_size = u32::MAX;
    tls_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()]; // this one is important

    let config = quinn::ServerConfig::with_crypto(std::sync::Arc::new(tls_config));

    Ok(config)
}
