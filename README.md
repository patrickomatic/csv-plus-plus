# gs-push-template 

Just a quick and dirty tool that pushes a local CSV file to a Google Spreadsheet, without overwriting data.  This allows you to write a spreadsheet using your text editor, check it into git etc then push it as a template to an actual Google Sheet with the values you're interested in.

## Setup

* [Install asdf](https://asdf-vm.com/guide/getting-started.html) and the curry ruby version in `.tool-versions`


* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`

* "Share" the spreadsheet with the email associated with the service account

## Usage

```
$ ./bin/gspush -k [..] my_template.csv
$ cat my_template.csv | ./bin/gspush -k [..]
```

## Template Language

The input follows all basic CSV rules, however the following variables will be provided:

* $$ROWNUM - 
