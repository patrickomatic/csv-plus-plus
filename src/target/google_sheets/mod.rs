//! # `GoogleSheets`
//!
// TODO:
// * implement backing up
//
mod batch_update_builder;
mod compilation_target;
mod credentials;
mod google_sheets_cell;

use super::{ExistingCell, ExistingValues};
use crate::{Compiler, Error, Module, Result};
use batch_update_builder::BatchUpdateBuilder;
use credentials::Credentials;
use google_sheets4::hyper;
use google_sheets4::hyper_rustls;
use google_sheets4::oauth2;
use log::{error, warn};

type SheetsHub = google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

type SheetsValue = google_sheets4::api::CellData;

pub(crate) struct GoogleSheets<'a> {
    async_runtime: tokio::runtime::Runtime,
    credentials: Credentials,
    compiler: &'a Compiler,
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
    pub(crate) fn new<S: Into<String>>(compiler: &'a Compiler, sheet_id: S) -> Result<Self> {
        let credentials = compiler.try_into()?;

        let async_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                Error::InitError(format!(
                    "Error starting async runtime to write Google Sheets: {e}"
                ))
            })?;

        Ok(Self {
            async_runtime,
            credentials,
            sheet_id: sheet_id.into(),
            compiler,
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
                match e {
                    google_sheets4::Error::BadRequest(obj)
                        if obj["error"]["code"].as_u64().is_some_and(|c| c == 404) =>
                    {
                        // 404 is fine, it just means it doesn't exist yet
                        return Ok(ExistingValues::default());
                    }
                    _ => {
                        warn!("Google Sheets API error response: {e}");

                        // TODO: show just the message
                        return Err(Error::GoogleSetupError(format!(
                            "Error reading existing sheet: {e}",
                        )));
                    }
                }
            }
        };

        /* TODO: ugh why won't this wor!!
        let row_data = spreadsheet
            .sheets
            .and_then(|sheets| sheets.get(0))
            .and_then(|sheet| sheet.data)
            .and_then(|data| data.get(0))
            .map(|grid_data| grid_data.row_data);

        let Some(row_data) = row_data.unwrap() else {
            return Ok(ExistingValues::default());
        };
        */

        let sheets = unwrap_or_empty!(spreadsheet.sheets); // Vec<Sheet>
        let sheet = unwrap_or_empty!(sheets.first()); // &Sheet
        let data = unwrap_or_empty!(&sheet.data); // &Vec<GridData>
        let grid_data = unwrap_or_empty!(data.first()); // &GridData
        let row_data = unwrap_or_empty!(&grid_data.row_data); // &Vec<RowData>

        let mut cells = vec![];

        for row in row_data {
            if let Some(v) = &row.values {
                cells.push(
                    v.iter()
                        .map(|cell| ExistingCell::Value(cell.clone()))
                        .collect(),
                );
            } else {
                cells.push(vec![]);
            }
        }

        Ok(ExistingValues { cells })
    }

    async fn sheets_hub(&self) -> Result<SheetsHub> {
        let auth = if self.credentials.is_authorized_user()? {
            let secret = oauth2::read_authorized_user_secret(&self.credentials.file)
                .await
                .map_err(|e| {
                    Error::GoogleSetupError(format!("Error reading application secret: {e}"))
                })?;

            oauth2::AuthorizedUserAuthenticator::builder(secret)
                .build()
                .await
                .map_err(|e| {
                    Error::GoogleSetupError(format!(
                        "Error requesting access to the spreadsheet: {e}"
                    ))
                })?
        } else if self.credentials.is_service_account()? {
            let secret = oauth2::read_service_account_key(&self.credentials.file)
                .await
                .map_err(|e| {
                    Error::GoogleSetupError(format!(
                        "Error reading sevice account credentials: {e}"
                    ))
                })?;

            oauth2::ServiceAccountAuthenticator::builder(secret)
                .build()
                .await
                .map_err(|e| {
                    Error::GoogleSetupError(format!(
                        "Error building service account authenticator: {e}"
                    ))
                })?
        } else {
            return Err(Error::GoogleSetupError(
                "Credentials file must be a service or user account".to_string(),
            ));
        };

        Ok(google_sheets4::Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        ))
    }

    async fn write_sheet(&self, module: &Module) -> Result<()> {
        let hub = self.sheets_hub().await?;
        let existing_values = self.read_existing_cells(&hub).await?;
        let batch_update_request =
            BatchUpdateBuilder::new(self.compiler, module, &existing_values).build();

        hub.spreadsheets()
            .batch_update(batch_update_request, &self.sheet_id)
            .doit()
            .await
            .map(|_i| ())
            .map_err(|e| {
                error!("Error writing to Google Sheets API: {e:?}");
                self.compiler
                    .output_error(format!("Error writing to Google Sheets: {e}"))
            })
    }
}

#[cfg(test)]
mod tests {
    /* TODO: need to
    use std::path;

    use super::*;
    use crate::CliArgs;

    fn build_compiler() -> Compiler {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        Compiler::new(cli_args).unwrap()
    }

    fn build_module() -> Module {
        Module::default()
    }

    #[test]
    fn write() {
        let module = build_module();
        let compiler = build_compiler();
        let target = GoogleSheets::new(&compiler, "1z1PQsfooud19mPwKcix3ocUpg9yXeiAXA2GycxWlpqU").unwrap();

        let result = target.write(&module);
        assert!(result.is_ok());
    }
    */
}
