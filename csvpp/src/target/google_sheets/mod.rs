//! # GoogleSheets
//!
use futures::executor;
use google_sheets4::hyper;
use google_sheets4::hyper_rustls;
use google_sheets4::oauth2;

use crate::{Result, Runtime, Template};
use super::CompilationTarget;

pub struct GoogleSheets<'a> {
    pub sheet_id: String,
    runtime: &'a Runtime,
}

type SheetsClient = google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

async fn sheets_client() -> SheetsClient {
    let secret: oauth2::ApplicationSecret = Default::default();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();

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

pub async fn write_sheet(_runtime: &Runtime, _template: &Template) -> Result<()> {
    let client = sheets_client().await;

    let req = google_sheets4::api::ValueRange::default();
    let result = client.spreadsheets().values_append(req, "spreadsheetId", "range")
             .value_input_option("dolor")
             .response_value_render_option("ea")
             .response_date_time_render_option("ipsum")
             .insert_data_option("invidunt")
             .include_values_in_response(true)
             .doit()
             .await;

    dbg!(result.unwrap());

    Ok(())
}

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        todo!();
    }

    fn write(&self, template: &Template) -> Result<()> {
        executor::block_on(write_sheet(self.runtime, template))
    }
}

impl<'a> GoogleSheets<'a> {
    pub fn new(runtime: &'a Runtime, sheet_id: &'a str) -> Self {
        Self {
            sheet_id: sheet_id.to_owned(),
            runtime,
        }
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
        GoogleSheets::new(&build_runtime(), "test-1234");
    }
}
