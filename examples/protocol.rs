use rsasl::{SASL};
use rsasl::error::SASLError;
use rsasl::session::Session;

struct ProtocolHandler {
    sasl_handler: SASL,
    authentication: Option<Session>,
}

impl ProtocolHandler {
    fn handle_auth(&mut self, mechs: &[&str]) -> Result<(), SASLError> {
        let mech = self.sasl_handler.suggest_client_mechanism(mechs.into_iter());
        if let Some(mech) = mech {
        }
        todo!()
    }
}

fn main() {
    let provider = SASL::new();

    let handler = ProtocolHandler {
        sasl_handler: provider,
        authentication: None,
    };
}