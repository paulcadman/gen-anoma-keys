use clap::Parser;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use shadow_rs::shadow;
use crate::build::CLAP_LONG_VERSION;

shadow!(build);

#[derive(Parser)]
#[command(name = "generate-anoma-keys")]
#[clap(long_version = CLAP_LONG_VERSION)]
#[command(about = "Generate ED25519 keys for use in an Anoma/Juvix program", long_about = None)]
struct Args {
    /// Name of the PublicKey type
    #[arg(long, default_value = "PublicKey")]
    publickey_type_name: String,

    /// Name of the PublicKey constructor
    #[arg(long, default_value = "PublicKey.mk")]
    publickey_ctor_name: String,

    /// Name of the PublicKey type
    #[arg(long, default_value = "PrivateKey")]
    privatekey_type_name: String,

    /// Name of the PrivateKey constructor
    #[arg(long, default_value = "PrivateKey.mk")]
    privatekey_ctor_name: String,
}

fn format_bytes_juvix(input: &[u8]) -> String {
    let joined_string = input
        .iter()
        .map(|byte| format!("0x{:02x}", byte))
        .collect::<Vec<String>>()
        .join("; ");
    format!("[{}]", joined_string)
}

fn main() {
    let args = Args::parse();

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    let keypair_bytes = signing_key.to_keypair_bytes();
    let public_key_bytes = signing_key.verifying_key().to_bytes();

    print!("privKey : {} := {} ", args.privatekey_type_name, args.privatekey_ctor_name);
    print!("{}", format_bytes_juvix(&keypair_bytes));
    println!(";");
    print!("pubKey : {}  := {} ", args.publickey_type_name, args.publickey_ctor_name);
    print!("{}", format_bytes_juvix(&public_key_bytes));
    println!(";");
}
