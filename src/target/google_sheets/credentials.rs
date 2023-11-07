use crate::{Error, Result, Runtime};
use std::env;
use std::fs;
use std::path;

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

impl TryFrom<&Runtime> for Credentials {
    type Error = Error;

    fn try_from(runtime: &Runtime) -> Result<Self> {
        let adc_path = adc_path()?;

        let creds_file = if let Some(creds) = &runtime.options.google_account_credentials {
            runtime.info("Using credentials from --google-account-credentials flag");
            path::PathBuf::from(creds)
        } else if let Some(env_var) = env::var_os("GOOGLE_APPLICATION_CREDENTIALS") {
            runtime.info("Using credentials from GOOGLE_APPLICATION_CREDENTIALS env var");
            path::PathBuf::from(env_var)
        } else if adc_path.exists() {
            runtime.info(format!(
                "Using credentials from ADC path: {}",
                adc_path.display()
            ));
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
    pub(super) fn is_service_account(&self) -> Result<bool> {
        Ok(self.read_json()?["type"] == "service_account")
    }

    pub(super) fn is_authorized_user(&self) -> Result<bool> {
        Ok(self.read_json()?["type"] == "authorized_user")
    }

    pub fn read_json(&self) -> Result<serde_json::Value> {
        let json = fs::read_to_string(&self.file).map_err(|e| {
            Error::GoogleSetupError(format!("Unable to read credentials file: {e}"))
        })?;

        serde_json::from_str(&json).map_err(|e| {
            Error::GoogleSetupError(format!("Error parsing credentials file JSON: {e}"))
        })
    }
}
