## gen-anoma-keys

A tool to generate ED25519 keys for use in an Anoma program.

### Installation

#### Prebuilt binaries

Use the prebuilt binaries for macOS-aarch64 and linux-x86_64 from the [latest release](https://github.com/paulcadman/gen-anoma-keys/releases/latest).

#### Compile from source

To compile from source install [cargo](https://rustup.rs) and run:

``` sh
cargo build --release
```

The exectuable will be available in `target/release/gen-anoma-keys`.

### Usage

``` sh
$ gen-anoma-keys
privKey : PrivateKey := PrivateKey.mk [0xa7; 0x23; 0x1a; 0xda; 0x00; 0x7f; 0x51; 0x98; 0x26; 0xd9; 0xeb; 0x0c; 0x00; 0xed; 0x03; 0xf5; 0xd2; 0x2f; 0x63; 0xd7; 0xb5; 0xb2; 0xa1; 0x7e; 0x94; 0x3b; 0x50; 0x85; 0xd6; 0x7f; 0x5f; 0xe3; 0x55; 0xb1; 0x8c; 0xd7; 0xaa; 0x3a; 0x6f; 0xd0; 0x08; 0x3e; 0xdd; 0x9f; 0xc3; 0x54; 0x97; 0x8d; 0xbe; 0x3f; 0xce; 0x1d; 0x90; 0x22; 0x9f; 0x5a; 0x37; 0xd9; 0x37; 0x28; 0x4e; 0x02; 0x11; 0x09];
pubKey : PublicKey  := PublicKey.mk [0x55; 0xb1; 0x8c; 0xd7; 0xaa; 0x3a; 0x6f; 0xd0; 0x08; 0x3e; 0xdd; 0x9f; 0xc3; 0x54; 0x97; 0x8d; 0xbe; 0x3f; 0xce; 0x1d; 0x90; 0x22; 0x9f; 0x5a; 0x37; 0xd9; 0x37; 0x28; 0x4e; 0x02; 0x11; 0x09];
```

The default type names `PublicKey`, `PrivateKey` and the constructor names `PublicKey.mk`, `PrivateKey.mk` can be configured with flags.

You can also pass your own hex-encoded 32 byte seed for the private key:

``` sh
$ gen-anoma-keys 91F8E7142DC0B4AA5C1A45BF7D05EBADF76FD91855D9C6F4B290F756E0B708C6
privKey : PrivateKey := PrivateKey.mk [0x91; 0xf8; 0xe7; 0x14; 0x2d; 0xc0; 0xb4; 0xaa; 0x5c; 0x1a; 0x45; 0xbf; 0x7d; 0x05; 0xeb; 0xad; 0xf7; 0x6f; 0xd9; 0x18; 0x55; 0xd9; 0xc6; 0xf4; 0xb2; 0x90; 0xf7; 0x56; 0xe0; 0xb7; 0x08; 0xc6; 0xa7; 0x1b; 0x04; 0x05; 0xb6; 0xba; 0x43; 0x4d; 0x69; 0xbf; 0xc0; 0x34; 0x5f; 0x23; 0x9d; 0xc4; 0xe5; 0xf1; 0xad; 0x73; 0x64; 0x55; 0x47; 0x73; 0x41; 0xf0; 0x91; 0xce; 0x5a; 0xac; 0x01; 0x34];
pubKey : PublicKey  := PublicKey.mk [0xa7; 0x1b; 0x04; 0x05; 0xb6; 0xba; 0x43; 0x4d; 0x69; 0xbf; 0xc0; 0x34; 0x5f; 0x23; 0x9d; 0xc4; 0xe5; 0xf1; 0xad; 0x73; 0x64; 0x55; 0x47; 0x73; 0x41; 0xf0; 0x91; 0xce; 0x5a; 0xac; 0x01; 0x34];
```

### Help

``` sh
Generate ED25519 keys for use in an Anoma/Juvix program

Usage: gen-anoma-keys [OPTIONS] [SEED]

Arguments:
  [SEED]  A private key seed

Options:
      --publickey-type-name <PUBLICKEY_TYPE_NAME>
          Name of the PublicKey type [default: PublicKey]
      --publickey-ctor-name <PUBLICKEY_CTOR_NAME>
          Name of the PublicKey constructor [default: PublicKey.mk]
      --privatekey-type-name <PRIVATEKEY_TYPE_NAME>
          Name of the PublicKey type [default: PrivateKey]
      --privatekey-ctor-name <PRIVATEKEY_CTOR_NAME>
          Name of the PrivateKey constructor [default: PrivateKey.mk]
  -h, --help
          Print help
  -V, --version
          Print version
```
