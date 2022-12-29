# csv++

A tool that allows you to work on enhanced CSV files locally in your favorite text editor, and write their results to existing Google Sheets.  This allows you to write a spreadsheet template, check it into git and push changes out to spreadsheets automatically.

## Setup

* [Install asdf](https://asdf-vm.com/guide/getting-started.html) and the current ruby version in `.tool-versions`
* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`
* "Share" the spreadsheet with the email associated with the service account

## Usage

```
$ ./bin/gspush -k [..] my_template.csv
$ cat my_template.csv | ./bin/gspush -k [..]
```

## Template Language

This program provides an enhanced language on top of CSV.  All of the normal rules of CSV apply, with the addition that each cell can have a:

```
foo,[[...]]bar,baz
```

specifying formatting or various other modifiers to the cell.  Additionally a row can start with:

```
![[...]]foo,bar,baz
```

which will apply that modifier to all cells in the row.

### Examples

* Align the second cell left, align the last cell to the center and make it bold and italicized:

```
Date,[[align=left]]Amount,Quantity,[[align=center/format=bold italic]]Price
```

* Underline and center-align an entire row:

```
![[align=center/format=underline]]Date,Amount,Quantity,Price
```

* A header for the first row, then some formulas that repeat for each row for the rest of the spreadsheet:

```
![[align=center/format=bold]]Date,Price,Quantity,Profit
![[expand=1:]],,,"=MULTIPLY(B$$ROW, C$$ROW)"
```

## Predefined variables

* `$$ROW` - The current row number.  The first row of the spreadsheet starts at 1
