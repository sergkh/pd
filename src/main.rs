use std::io::{self, BufRead};
use base64::{Engine as _, engine::general_purpose}; // https://docs.rs/base64/latest/base64/
use argon2::Argon2; // https://docs.rs/argon2/latest/argon2/
use clap::Parser;
use atty::Stream;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// specify the password domain, will be used as salt
    #[arg(short, long)]
    domain: Option<String>
}

fn main() {
    let args = Cli::parse();
    let salt = if let Some(d) = args.domain.as_deref() {
        // PAD to at least 16 chars
        format!("{:0<16}", d.to_string()) 
    } else {
        "default domain01".to_string()
    };

    let password: String = match atty::is(Stream::Stdin) {
        true => {            
            rpassword::prompt_password("Password: ").unwrap() // read masked password from user input
        },
        false => {
            // read password from stdin, e.g. `echo "password" | pd`  
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).unwrap();
            line.trim_end().to_string()
        }
        
    };

    let mut output_key_material = [0u8; 32];    
    let _ = Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output_key_material);

    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(output_key_material);
    
    println!("{}", encoded);
}