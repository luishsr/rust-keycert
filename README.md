# rust-keycert

Self-Signed Certificate Generator in Rust

![image](https://github.com/luishsr/rust-keycert/assets/80909424/d4cf6bf6-3bd9-4c8a-80a4-d9e784903d59)

This Rust-based utility provides an easy and efficient way to generate self-signed ECDSA certificates for local development and testing purposes. Wrapped around the power of OpenSSL, this tool ensures the creation of secure certificates, aptly designed for local HTTPS servers, development, and testing environments.
Features 🌟

    ECDSA-based Certificates: Utilizes the Elliptic Curve Digital Signature Algorithm for modern, secure, and efficient certificates.
    PEM Formatted Output: Generates both the private key and certificate in the widely accepted PEM format.
    Pre-configured for localhost: Ideal for local web development and testing.
    Validation Tools: Includes helper functions and utilities for validating the generated certificates.

Getting Started 🚀
Prerequisites

    Rust & Cargo: Ensure you have Rust and Cargo installed.
    OpenSSL: Make sure you have the OpenSSL development libraries installed.

Building from Source

    git clone https://github.com/luishsr/rust-keycert.git
    cd repository_name
    cargo build --release

Usage

Once built, run the utility:

    ./target/release/certificate_generator

This will generate a localhost.key (private key) and localhost.crt (certificate) in the current directory.
Testing & Validation 🧪

Refer to our Testing Guide for steps on how to validate and test the generated certificates.
Contributing 🤝

All contributions, from beginners to experts, are welcome! Check out the Contributing Guide for details on how to start contributing.
License 📜

This project is licensed under the MIT License. See the LICENSE file for details.

Crafted by Luis Soares. Feel free to star ⭐ the repository if this project helped you!
