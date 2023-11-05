//! # GoogleSheets
//!
// TODO:
// * implement backing up
//
// * better error handling throughout (cleanup unwrap()s)
//
mod batch_update_builder;
mod compilation_target;
mod google_sheets_modifier;

use super::{ExistingCell, ExistingValues};
use crate::{Error, Result, Runtime, Template};
use batch_update_builder::BatchUpdateBuilder;
use google_sheets4::hyper;
use google_sheets4::hyper_rustls;
use google_sheets4::oauth2;
use std::env;
use std::path;

type SheetsHub = google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

type SheetsValue = google_sheets4::api::CellData;

pub(crate) struct GoogleSheets<'a> {
    async_runtime: tokio::runtime::Runtime,
    credentials: path::PathBuf,
    runtime: &'a Runtime,
    pub(crate) sheet_id: String,
}

macro_rules! unwrap_or_empty {
    ($to_unwrap:expr) => {{
        match $to_unwrap {
            Some(s) => s,
            None => return Ok(ExistingValues::default()),
        }
    }};
}

impl<'a> GoogleSheets<'a> {
    pub(crate) fn new<S: Into<String>>(runtime: &'a Runtime, sheet_id: S) -> Result<Self> {
        let credentials = Self::get_credentials(runtime)?;

        let async_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                runtime.output.clone().into_error(format!(
                    "Error starting async runtime to write Google Sheets: {e}"
                ))
            })?;

        Ok(Self {
            async_runtime,
            credentials,
            sheet_id: sheet_id.into(),
            runtime,
        })
    }

    async fn read_existing_cells(&self, hub: &SheetsHub) -> Result<ExistingValues<SheetsValue>> {
        let spreadsheet = match hub
            .spreadsheets()
            .get(&self.sheet_id)
            .include_grid_data(true)
            .doit()
            .await
        {
            Ok((_, s)) => s,
            Err(e) => {
                self.runtime
                    .info(format!("Google Sheets API response: {e}"));

                match e {
                    google_sheets4::Error::BadRequest(obj)
                        if obj["error"]["code"].as_u64().is_some_and(|c| c == 404) =>
                    {
                        // 404 is fine, it just means it doesn't exist yet
                        return Ok(ExistingValues::default());
                    }
                    google_sheets4::Error::BadRequest(obj)
                        if obj["error"]["code"].as_u64().is_some_and(|c| c == 403) =>
                    {
                        return Err(self
                            .runtime
                            .output
                            .clone()
                            .into_error("Unable to access the given spreadsheet - are you sure you shared it with your service account?"));
                    }
                    _ => {
                        return Err(self
                            .runtime
                            .output
                            .clone()
                            .into_error(format!("Error reading existing sheet: {e}")));
                    }
                }
            }
        };

        // TODO: ugh why can't I just chain .and_thens or .maps or something
        /*
        let row_data = spreadsheet
            .sheets
            .and_then(|sheets| sheets.get(0))
            .and_then(|sheet| sheet.data)
            .and_then(|data| data.get(0))
            .map(|&grid_data| grid_data.row_data);
        */

        let sheets = unwrap_or_empty!(spreadsheet.sheets);
        let sheet = unwrap_or_empty!(sheets.get(0));
        let data = unwrap_or_empty!(&sheet.data);
        let grid_data = unwrap_or_empty!(data.get(0));
        let row_data = unwrap_or_empty!(&grid_data.row_data);

        let mut existing_cells = vec![];
        for row in row_data.iter() {
            if let Some(v) = &row.values {
                existing_cells.push(
                    v.iter()
                        .map(|cell| ExistingCell::Value(cell.clone()))
                        .collect(),
                );
            } else {
                existing_cells.push(vec![]);
            }
        }

        Ok(ExistingValues {
            cells: existing_cells,
        })
    }

    fn get_credentials(runtime: &'a Runtime) -> Result<path::PathBuf> {
        let home_path =
            home::home_dir().ok_or(Error::InitError("Unable to get home directory".to_string()))?;

        let adc_path = home_path
            .join(".config")
            .join("gcloud")
            .join("application_default_credentials.json");

        let creds_file = if let Some(creds) = &runtime.options.google_account_credentials {
            path::PathBuf::from(creds)
        } else if let Some(env_var) = env::var_os("GOOGLE_APPLICATION_CREDENTIALS") {
            path::PathBuf::from(env_var)
        } else if adc_path.exists() {
            adc_path
        } else {
            return Err(runtime.output.clone().into_error(
                    "Could not find Google application credentials.  You must create a service account with \
                    access to your spreadsheet and supply the credentials via $GOOGLE_APPLICATION_CREDENTIALS, \
                    --google-account-credentials or putting them in \
                    ~/.config/gcloud/application_default_credentials.json"));
        };

        Ok(creds_file)
    }

    async fn sheets_hub(&self) -> SheetsHub {
        let secret = oauth2::read_service_account_key(&self.credentials)
            .await
            .expect("Error reading service account key");

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
                    .build(),
            ),
            auth,
        )
    }

    async fn write_sheet(&self, template: &Template<'a>) -> Result<()> {
        let hub = self.sheets_hub().await;
        let existing_values = self.read_existing_cells(&hub).await?;
        let batch_update_request =
            BatchUpdateBuilder::new(self.runtime, template, &existing_values).build();

        hub.spreadsheets()
            .batch_update(batch_update_request, &self.sheet_id)
            .doit()
            .await
            .map(|_i| ())
            .map_err(|e| {
                self.runtime.warn(format!("{:?}", e));
                self.runtime
                    .output
                    .clone()
                    .into_error(format!("Error writing to Google Sheets: {e}"))
            })
    }
}

#[cfg(test)]
mod tests {
    /* TODO: need to
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
        let target = GoogleSheets::new(&runtime, "1z1PQsfooud19mPwKcix3ocUpg9yXeiAXA2GycxWlpqU").unwrap();

        let result = target.write(&template);
        assert!(result.is_ok());
    }
    */
}
