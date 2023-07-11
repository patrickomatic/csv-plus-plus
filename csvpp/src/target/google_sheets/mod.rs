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

mod batch_update_builder;
mod google_sheets_modifier;

use batch_update_builder::BatchUpdateBuilder;

type SheetsHub = google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

#[derive(Debug)]
pub enum CellValue { 
    ExistingValue(google_sheets4::api::ExtendedValue),
    Empty,
}

#[derive(Debug, Default)]
pub struct ExistingValues {
    pub cells: Vec<Vec<CellValue>>,
}

impl ExistingValues {
    pub async fn read(hub: &SheetsHub, sheet_id: &str) -> Result<ExistingValues> {
        Self::read_existing_cells(hub, sheet_id).await
    }

    async fn read_existing_cells(hub: &SheetsHub, sheet_id: &str) -> Result<ExistingValues> {
        let request = hub
            .spreadsheets()
            .get(sheet_id)
            .include_grid_data(true)
            .doit()
            .await;

        let empty = Ok(Self::default());

        let spreadsheet = match request {
            Ok((_, s)) => s,
            Err(e) => match e {
                // not necessarily unexpected - the target just doesn't exist yet (returned 404)
                google_sheets4::Error::BadRequest(obj) if obj["error"]["code"].as_u64().is_some_and(|c| c == 404) 
                    => return empty,
                _ 
                    => return Err(Error::InitError(format!("Error reading existing sheet: {}", e))),
            },
        };

        // everything in this API is an Option<> so has to be unwrapped...
        //
        let sheets = match spreadsheet.sheets {
            Some(s) => s,
            None => return empty,
        };

        let sheet = match sheets.get(0) {
            Some(s) => s,
            None => return empty,
        };

        let data = match &sheet.data {
            Some(d) => d,
            None => return empty,
        };

        let grid_data = match data.get(0) {
            Some(d) => d,
            None => return empty,
        };

        let row_data = match &grid_data.row_data {
            Some(d) => d,
            None => return empty,
        };

        let mut existing_cells = vec![];
        for row in row_data.iter() {
            match &row.values {
                Some(v) => {
                    existing_cells.push(v.iter().map(|cell| {
                        match &cell.user_entered_value {
                            Some(v) => CellValue::ExistingValue(v.clone()),
                            None => CellValue::Empty,
                        }
                    }).collect());
                },
                None => existing_cells.push(vec![]),
            }
        }

        Ok(Self { cells: existing_cells })
    }

    // A shorter/more direct way of getting just the formula values.  This will not include any
    // cell data which includes stuff like formatting
    /*
    async fn read_formula_values(hub: &SheetsHub, sheet_id: &str) -> Result<Vec<Vec<CellValue>>> {
        let (_, body) = hub
            .spreadsheets()
            .values_get(sheet_id, "A1:Z1000")
            .value_render_option("FORMULA")
            .doit()
            .await
            .map_err(|e| Error::InitError(
                    format!("Error reading existing sheet: {}", e)))?;

        dbg!(body);
        let ret = vec![vec![]];
        Ok(ret)
    }
    */
}

pub struct GoogleSheets<'a> {
    async_runtime: tokio::runtime::Runtime,
    credentials: path::PathBuf,
    runtime: &'a Runtime,
    pub sheet_id: String,
}

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        // TODO I think I need a drive client actually 
        // let r = hub.spreadsheets().sheets_copy_to(...).doit().await
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
            .build()
            // TODO: use TargetError?
            .map_err(|e| Error::InitError(
                    format!("Error starting async runtime to write Google Sheets: {}", e)))?;

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
            return Err(Error::InitError(
                    "Could not find Google application credentials.  You must create a service account with \
                    access to your spreadsheet and supply the credentials via $GOOGLE_APPLICATION_CREDENTIALS, \
                    --google-account-credentials or putting them in \
                    ~/.config/gcloud/application_default_credentials.json".to_owned()))
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
                    .build()), 
            auth)
    }

    async fn write_sheet(&self, template: &Template<'a>) -> Result<()> {
        let hub = self.sheets_hub().await;

        // XXX maybe move the read up and out of this function?
        let existing_values = ExistingValues::read(&hub, &self.sheet_id).await?;

        let batch_update_request = BatchUpdateBuilder::new(self.runtime, template).build();
        let response = hub
            .spreadsheets()
            .batch_update(batch_update_request, &self.sheet_id)
            .doit()
            .await
            .expect("Error making batch update request");

        Ok(())
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
    
    /*
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
