use calimero_primitives::identity::PrivateKey;
use calimero_server_primitives::admin::GenerateContextIdentityResponse;
use clap::Parser;
use comfy_table::{Cell, Color, Table};
use eyre::Result as EyreResult;

use super::temp_identity_store::{store_identity, IdentityPair};
use crate::cli::Environment;
use crate::output::Report;

#[derive(Debug, Parser)]
#[command(about = "Generate public/private key pair used for context identity")]
pub struct GenerateCommand;

impl Report for GenerateContextIdentityResponse {
    fn report(&self) {
        let mut table = Table::new();
        let _ = table.set_header(vec![Cell::new("Generated Identity").fg(Color::Blue)]);
        let _ = table.add_row(vec![format!("Public Key: {}", self.data.public_key)]);
        let _ = table.add_row(vec![format!("Private Key: {}", self.data.private_key)]);
        println!("{table}");
    }
}

impl GenerateCommand {
    pub async fn run(self, environment: &Environment) -> EyreResult<()> {
        let private_key = PrivateKey::random(&mut rand::thread_rng());
        let public_key = private_key.public_key();

        let identity = IdentityPair {
            public: public_key.clone(),
            private: private_key.clone(),
        };

        for slot in 0u8..32 {
            if store_identity(slot, identity.clone()) {
                break;
            }
        }

        let response = GenerateContextIdentityResponse::new(private_key.public_key(), private_key);
        environment.output.write(&response);

        Ok(())
    }
}
