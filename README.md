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

### Help

```
Generate ED25519 keys for use in an Anoma/Juvix program

Usage: gen-anoma-keys [OPTIONS]

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
