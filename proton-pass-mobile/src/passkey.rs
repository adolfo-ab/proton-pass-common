pub use proton_pass_common::passkey::PasskeyError;
use proton_pass_common::passkey::{generate_passkey_for_domain, resolve_challenge_for_domain, PasskeyResult};

pub struct CreatePasskeyResponse {
    pub passkey: Vec<u8>,
    pub response: String,
    pub rp_name: String,
    pub user_name: String,
    pub user_display_name: String,
}

pub struct PasskeyManager {
    rt: tokio::runtime::Runtime,
}

impl PasskeyManager {
    pub fn new() -> PasskeyResult<Self> {
        match tokio::runtime::Builder::new_current_thread().build() {
            Ok(rt) => Ok(Self { rt }),
            Err(e) => Err(PasskeyError::RuntimeError(format!("Error creating runtime: {:?}", e))),
        }
    }

    pub fn generate_passkey(&self, url: String, request: String) -> PasskeyResult<CreatePasskeyResponse> {
        self.rt.handle().block_on(async move {
            match generate_passkey_for_domain(&url, &request).await {
                Ok(r) => match r.response() {
                    Ok(response) => Ok(CreatePasskeyResponse {
                        passkey: r.passkey,
                        response,
                        rp_name: r.rp_name,
                        user_display_name: r.user_display_name,
                        user_name: r.user_name,
                    }),
                    Err(e) => {
                        println!("Error in generate_passkey: {:?}", e);
                        Err(e)
                    }
                },
                Err(e) => {
                    println!("Error in generate_passkey: {:?}", e);
                    Err(e)
                }
            }
        })
    }

    pub fn resolve_challenge(&self, url: String, passkey: Vec<u8>, request: String) -> PasskeyResult<String> {
        self.rt.handle().block_on(async move {
            match resolve_challenge_for_domain(&url, &passkey, &request).await {
                Ok(r) => match r.response() {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        println!("Error in resolve_challenge: {:?}", e);
                        Err(e)
                    }
                },
                Err(e) => {
                    println!("Error in resolve_challenge: {:?}", e);
                    Err(e)
                }
            }
        })
    }
}