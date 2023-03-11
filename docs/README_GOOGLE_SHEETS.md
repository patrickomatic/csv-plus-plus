## Publishing to Google Sheets

Google Sheets requires you to set up a service account (for which you must add the credentials to your CLI) and then you must provide access to that service account.

* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`
* "Share" the spreadsheet with the email associated with the service account


