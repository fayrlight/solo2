#![no_main]
#![no_std]

/*
 * Copyright (c) NitroKey GmbH. 2021
 * Some rights reserved.
 *
 * fairlight <fairlight@nitrokey.com>
 *
 * Nitro Smartcard OpenPGP Smardcard v3.4 
 * Trussed Firmware Framework based Impl
 *
 * Functional implementation of OpenPGP functionality,
 * between ISO smart card and operating system.
 * The term "card" refers to the NitroKey v3 Security Stick.
 *
 */

const CRYPTO_RSA: usize 0x23;
const CRYPTO_ECC: usize 0x42;

pub struct Crypto {
    pub name: String,
    pub type: u8
};


