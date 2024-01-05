pub use proton_pass_common::totp::error::TOTPError;
use proton_pass_common::totp::sanitizer::{uri_for_editing, uri_for_saving};
pub use proton_pass_common::totp::totp::TotpTokenResult;
pub use proton_pass_common::totp::totp::TOTP;

pub struct TotpUriParser;
pub struct TotpUriSanitizer;

pub type TOTPAlgorithm = proton_pass_common::totp::algorithm::Algorithm;

impl TotpUriParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, uri_string: String) -> Result<TOTP, TOTPError> {
        TOTP::from_uri(uri_string.as_str())
    }
}

impl TotpUriSanitizer {
    pub fn new() -> Self {
        Self
    }

    pub fn uri_for_editing(&self, original_uri: String) -> String {
        uri_for_editing(original_uri.as_str())
    }

    pub fn uri_for_saving(&self, original_uri: String, edited_uri: String) -> Result<String, TOTPError> {
        uri_for_saving(original_uri.as_str(), edited_uri.as_str())
    }
}

pub struct TotpTokenGenerator;

impl TotpTokenGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_token(&self, uri: String, current_time: u64) -> Result<TotpTokenResult, TOTPError> {
        let totp: TOTP = if uri.contains("otpauth") {
            TOTP::from_uri(&uri)?
        } else {
            TOTP {
                secret: uri,
                ..Default::default()
            }
        };
        let token = totp.generate_token(current_time)?;
        Ok(TotpTokenResult {
            totp,
            token,
            timestamp: current_time,
        })
    }
}
