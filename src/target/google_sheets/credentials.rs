use super::HyperConnector;
use crate::{Compiler, Error, Result};
use google_sheets4::yup_oauth2;
use log::info;
use std::{env, fs, path};

/// The file containing credentials that will be used to connect to Sheets API
#[derive(Debug)]
pub(super) struct Credentials {
    pub(super) file: path::PathBuf,
}

fn adc_path() -> Result<path::PathBuf> {
    if cfg!(target_family = "windows") {
        let Some(app_data) = env::var_os("APPDATA") else {
            return Err(Error::InitError(
                "Unable to resolve %APPDATA% environment variable".to_string(),
            ));
        };

        Ok(path::Path::new(&app_data)
            .join("gcloud")
            .join("application_default_credentials.json"))
    } else if cfg!(target_family = "unix") {
        Ok(home::home_dir()
            .ok_or(Error::InitError("Unable to get home directory".to_string()))?
            .join(".config")
            .join("gcloud")
            .join("application_default_credentials.json"))
    } else {
        // we don't support this target_family - just windows and unix
        unimplemented!("Unsupported target: unknown application_default_credentials.json location")
    }
}

impl TryFrom<&Compiler> for Credentials {
    type Error = Error;

    fn try_from(compiler: &Compiler) -> Result<Self> {
        let adc_path = adc_path()?;

        let creds_file = if let Some(creds) = &compiler.config.google_account_credentials {
            info!("Using credentials from --google-account-credentials flag");
            path::PathBuf::from(creds)
        } else if let Some(env_var) = env::var_os("GOOGLE_APPLICATION_CREDENTIALS") {
            info!("Using credentials from GOOGLE_APPLICATION_CREDENTIALS env var");
            path::PathBuf::from(env_var)
        } else if adc_path.exists() {
            info!("Using credentials from ADC path: {}", adc_path.display());
            adc_path
        } else {
            // TODO: more words in this error
            return Err(Error::GoogleSetupError(
                "Could not find any suitable Google app credentials".to_string(),
            ));
        };

        if !creds_file.exists() {
            return Err(Error::GoogleSetupError(format!(
                "Credentials file does not exist: {}",
                creds_file.display()
            )));
        }

        Ok(Self { file: creds_file })
    }
}

impl Credentials {
    pub(super) fn read_json(&self) -> Result<serde_json::Value> {
        let json = fs::read_to_string(&self.file).map_err(|e| {
            Error::GoogleSetupError(format!("Unable to read credentials file: {e}"))
        })?;

        serde_json::from_str(&json).map_err(|e| {
            Error::GoogleSetupError(format!("Error parsing credentials file JSON: {e}"))
        })
    }

    pub(super) async fn auth(
        &self,
    ) -> Result<yup_oauth2::authenticator::Authenticator<HyperConnector>> {
        let json = self.read_json()?;

        if json["type"] == "authorized_user" {
            Ok(yup_oauth2::AuthorizedUserAuthenticator::builder(
                yup_oauth2::read_authorized_user_secret(&self.file)
                    .await
                    .map_err(|e| {
                        Error::GoogleSetupError(format!("Error reading application secret: {e}"))
                    })?,
            )
            .build()
            .await
            .map_err(|e| {
                Error::GoogleSetupError(format!("Error requesting access to the spreadsheet: {e}"))
            })?)
        } else if json["type"] == "service_account" {
            Ok(yup_oauth2::ServiceAccountAuthenticator::builder(
                yup_oauth2::read_service_account_key(&self.file)
                    .await
                    .map_err(|e| {
                        Error::GoogleSetupError(format!(
                            "Error reading sevice account credentials: {e}"
                        ))
                    })?,
            )
            .build()
            .await
            .map_err(|e| {
                Error::GoogleSetupError(format!(
                    "Error building service account authenticator: {e}"
                ))
            })?)
        } else {
            Err(Error::GoogleSetupError(
                "Credentials file must be a service or user account but saw type: ".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::env;

    #[test]
    #[ignore]
    fn try_from_default_location() {
        // hard to do this one because it checks ~/.config/gcloud/...
        todo!()
    }

    #[test]
    fn try_from_env_var() {
        let test_file = TestFile::new("json", "{\"type\": \"service_account\"}");
        env::set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            test_file.path.to_string_lossy().to_string(),
        );
        let compiler = build_compiler();

        assert!(Credentials::try_from(&compiler).is_ok());
    }

    #[test]
    fn try_from_config() {
        let test_file = TestFile::new("json", "{\"type\": \"service_account\"}");
        let mut compiler = build_compiler();
        compiler.config.google_account_credentials = Some(
            test_file
                .path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
        );

        assert!(Credentials::try_from(&compiler).is_ok());
    }

    #[test]
    #[ignore]
    fn try_from_does_not_exist() {
        // hard to test this because it will always catch the default application creds locally
        todo!()
    }
}
