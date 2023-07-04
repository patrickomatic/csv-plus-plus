//! # GoogleSheets
//!
// TODO: 
// * better error handling throughout (cleanup unwrap()s)
//
use google_sheets4::hyper;
use google_sheets4::hyper_rustls;
use google_sheets4::oauth2;
use std::env;
use std::path;
use crate::{Error, Result, Runtime, Template};
use super::CompilationTarget;

pub struct GoogleSheets<'a> {
    async_runtime: tokio::runtime::Runtime,
    credentials: path::PathBuf,
    runtime: &'a Runtime,
    pub sheet_id: String,
}

type SheetsClient = google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;


impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        // TODO
        todo!();
    }

    fn write(&self, template: &Template) -> Result<()> {
        self.async_runtime.block_on(async {
            self.write_sheet(template).await
        })
    }
}

impl<'a> GoogleSheets<'a> {
    pub fn new(runtime: &'a Runtime, sheet_id: &'a str) -> Result<Self> {
        let credentials = Self::get_credentials(runtime)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build().unwrap();

        Ok(Self {
            async_runtime: rt,
            credentials,
            sheet_id: sheet_id.to_owned(),
            runtime,
        })
    }

    fn get_credentials(runtime: &'a Runtime) -> Result<path::PathBuf> {
        let home_path = home::home_dir().ok_or(
            Error::InitError("Unable to get home directory".to_string()))?;

        let adc_path = home_path.join(".config")
            .join("gcloud")
            .join("application_default_credentials.json");

        let creds_file = if let Some(creds) = &runtime.options.google_account_credentials {
            path::PathBuf::from(creds)
        } else if let Some(env_var) = env::var_os("GOOGLE_APPLICATION_CREDENTIALS") {
            path::PathBuf::from(env_var)
        } else if adc_path.exists() {
            adc_path
        } else {
            return Err(Error::InitError("Could not find Google application credentials.  You must create a service account with access to your spreadsheet and supply the credentials via $GOOGLE_APPLICATION_CREDENTIALS, --google-account-credentials or putting them in ~/.config/gcloud/application_default_credentials.json".to_owned()))
        };

        Ok(creds_file)
    }

    async fn write_sheet(&self, _template: &Template) -> Result<()> {
        let client = self.sheets_client().await;

        let result = client.spreadsheets()
            .values_get(&self.sheet_id, "A1:Z1000")
            .doit()
            .await;

        dbg!(result.unwrap());

        Ok(())
    }

    async fn sheets_client(&self) -> SheetsClient {
        let secret = oauth2::read_service_account_key(&self.credentials)
            .await
            .expect("Erorr reading service account key");

        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .expect("Error building service account authenticator");

        google_sheets4::Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build()), 
            auth)
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;
    use crate::CliArgs;

    fn build_runtime() -> Runtime {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        Runtime::new(cli_args).unwrap()
    }
    
    fn build_template() -> Template {
        Template::default()
    }

    #[test]
    fn write() {
        let template = build_template();
        let runtime = build_runtime();
        let target = GoogleSheets::new(&runtime, "1tgvyN8cgMPx4pKa7LU5n0BW3JQa8wlsCI8quWpfPYFw").unwrap();

        assert!(target.write(&template).is_ok());
        assert!(false);
    }
}
