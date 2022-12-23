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

This program provides an ehanced language on top of CSV:

You can apply formatting to individual cells:
```
Date,<[align=left]>Amount,Quantity,<[align=center/format=bold italic]>Price
```

or to the entire row:
```
<![align=center/format=underline]>Date,Amount,Quantity,Price
```

## Predefined variables

* `$$ROWNUM` - The current row number.  The first row of the spreadsheet starts at 1
