# csv++

A tool that allows you to programtically author spreadsheets in your favorite text editor and write their results to existing Google Sheets (in the future it can probably support Excel).  This allows you to write a spreadsheet template, check it into git and push changes out to spreadsheets automatically.

## Setup

* [Install asdf](https://asdf-vm.com/guide/getting-started.html) and the current ruby version in `.tool-versions`
* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`
* "Share" the spreadsheet with the email associated with the service account

## Usage

```
# apply my_taxes_template.csvpp to an existing Google Sheet with name "Taxes 2022"
$ csv++ --sheet-name "Taxes 2022" --sheet-id "[...]" my_taxes_template.csvpp

# take input from stdin, supply a variable ($$rate = 1) and apply to the "Stocks" spreadsheet
$ cat stocks.csvpp | csv++ -k "rate=1" -n "Stocks" -i "[...]"
```

## Template Language

This program provides an enhanced language on top of CSV.  All of the normal rules of CSV apply, with the addition that each cell can have modifiers (specified in `[[`/`]]` for cells and `![[`/`]]` for rows):

```
foo,[[...]]bar,baz
```

specifying formatting or various other modifiers to the cell.  Additionally a row can start with:

```
![[...]]foo,bar,baz
```

which will apply that modifier to all cells in the row.

You can also define a code section at the top of the CSV to put shared variables and calculations:

```
fees := 0.65 # my broker charges $0.65 a trade

is_call := EQ(A$$rownum, "call")
commission := IFS($$is_call, ADD(C$$rownum * $$fees, A$$rownum))

---
![[format=bold/align=center]]Date,Purchase,Price,Quantity,Total,Fees
![[expand]],[[format=bold]],,,"=PROFIT(C$$rownum, D$$rownum)",$$commission
```

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

* `$$rownum` - The current row number.  The first row of the spreadsheet starts at 1

## CLI Arguments

* `-i, --sheet-id SHEET_ID` The id of the sheet - you can extract this from the URL: https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=0
* `-n, --sheet-name SHEET_NAME` The name of the sheet to apply the template to
* `-k, --key-values KEY_VALUES` A comma-separated list of key=values which will be made available to the template
* `-y, --offset-rows OFFSET` Apply the template offset by OFFSET rows
* `-x, --offset-columns OFFSET` Apply the template offset by OFFSET cells
* `-v, --verbose` Enable verbose output
* `-h, --help` Show help information
