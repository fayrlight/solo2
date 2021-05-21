# Nitro-Smartcard

Modern Trussed OpenPGP Smartcard v3.4 implementation for NitroKey v3 NFC

## Status

WiP. Actively developed. Stable APIs.

## Resources

### Trussed Rust Firmware Framework

* Trussed Firemware Framework <https://trussed.dev>
* Trussed oath-authenticator for OTP over CCID <https://github.com/trussed-dev/oath-authenticator>
* Trussed piv-authenticator FIPS 201 PIV protocol over CCID <https://github.com/solokeys/solo2/tree/main/components/piv-authenticator>

### Cryptographic References & Specification
* OpenPGP Smartcard v.34 <https://gnupg.org/ftp/specs/OpenPGP-smart-card-application-3.4.pdf>

### Communication
* CCID interface for USB communication <https://usb.org/sites/default/files/DWG_Smart-Card_CCID_Rev110.pdf>
* Virtual Smart Card Emulator with PC/SC <https://frankmorgner.github.io/vsmartcard/virtualsmartcard/README.html>

### Context NitroKey
* NitroKey Secure Element SE050 <https://www.nxp.com/docs/en/data-sheet/SE050-DATASHEET.pdf>
* STM32 MCU using NitroKey Start <https://www.nitrokey.com/files/doc/Nitrokey_Start_factsheet.pdf>

### C Precedence
* Gnuk C OpenSmart Card Implementation <http://git.gniibe.org/gitweb/?p=gnuk/gnuk.git;a=tree>
* Gnuk Experimental C to Rust Transpile <https://github.com/fayrlight/s2s-rust-gnuk>
