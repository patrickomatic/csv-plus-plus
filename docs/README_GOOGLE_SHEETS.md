## Publishing to Google Sheets

Google Sheets requires you to set up a service account (for which you must add the credentials to your CLI) and then you must provide access to that service account.

* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`
* "Share" the spreadsheet with the email associated with the service account - go to the spreadsheet and in the top right, share the document and paste in the email address of the service account.

## Writing RSpec Tests for Google Sheets

Writing tests for Google Sheets specs is a little more difficult because Google Sheets is an API rather than writing to a file.  This means we'll want to use [VCR](https://github.com/vcr/vcr) to record the requests, but being careful that all PII is stripped out:

1. Make sure your `.env` file is configured to point to a Google Sheet ID where the results can be written.
```
cp .env.sample .env
... edit and fill in your GOOGLE_SHEET_ID ...
```
2. If the test already exists, remove any `vcr:` directives and call `VCR.turn_off!`:
```
-    describe 'modifiers', vcr: { match_requests_on: [google_sheets_path_matcher] } do
+    describe 'modifiers' do
       context 'format=' do
         let(:rows) { [row] }
         let(:row) do
@@ -57,6 +58,8 @@ describe ::CSVPlusPlus::Writer::GoogleSheets do
         end

         it 'successfully writes the spreadsheet' do
+          ::WebMock.allow_net_connect!
+          ::VCR.turn_off!
           expect { subject }
             .not_to(raise_error)
         end
```
3. Run the test against the live Google Sheets API and verify the results are what you expect.
4. XXX re-enable the api request and record?
5. Before committing the cassettes make sure that they don't contain any PII or secret tokens.  If you need to, you can configure VCR to filter sensitive information in [../spec/spec\_helper.rb](spec\_helper.rb).
