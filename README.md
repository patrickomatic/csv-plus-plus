[![Ruby Style Guide](https://img.shields.io/badge/code_style-community-brightgreen.svg)](https://rubystyle.guide)
[![Gem Version](https://badge.fury.io/rb/csv_plus_plus.svg)](https://badge.fury.io/rb/csv_plus_plus)

# csv++

A tool that allows you to programatically author spreadsheets in your favorite text editor and write their results to CSV, Google Sheets, Excel and other spreadsheet formats.  This allows you to write a spreadsheet template, check it into git and push changes out to spreadsheets using typical dev tools.

## Template Language

A `csvpp` file consists of a (optional) code section and a CSV section separated by `---`.  In the code section you can define variables and functions that can be used in the CSV below it.  For example:

```
fees := 0.50 # my broker charges $0.50 a trade

price := cellref(C)
quantity := cellref(D)

def profit() (price * quantity) - fees

---
![[format=bold/align=center]]Date,Ticker,Price,Quantity,Total,Fees
![[expand]],[[format=bold]],,,"=PROFIT()",$$fees
```

## Variables

Variables can be defined in the code section by giving a name (a combination of letters, numbers and underscores ) the expression `:=` and followed with a value.

### Built-in Variables

* `$$rownum` - The current row number.  The first row of the spreadsheet starts at 1.  Can be used anywhere and it's value will evaluate to the current row being processed.

## Functions

### Built-in Functions
* `cellref(CELL)` - Returns a reference to the `CELL` relative to the current row.  If the current `$$rownum` is `2`, then `CELLREF("C")` returns  a reference to cell `C2`.

## Modifiers

Modifiers can change the formatting of a cell or row, apply validation, change alignment, etc. All of the normal rules of CSV apply, with the addition that each cell can have modifiers (specified in `[[`/`]]` for cells and `![[`/`]]` for rows):

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
![[expand=1:]],,,"=MULTIPLY(cellref(B), cellref(C))"
```

## Setup (Google Sheets)

Just install it via rubygems (homebrew and debian packages are in the works):

`$ gem install csv_plus_plus`

### Publishing to Google Sheets

* Go to the [GCP developers console](https://console.cloud.google.com/projectselector2/apis/credentials?pli=1&supportedpurview=project), create a service account and export keys for it to `~/.config/gcloud/application_default_credentials.json`
* "Share" the spreadsheet with the email associated with the service account

## CLI Arguments

```
Usage: csv++ [options]
    -b, --backup                     Create a backup of the spreadsheet before applying changes.
    -g, --google-sheet-id SHEET_ID   The id of the sheet - you can extract this from the URL: https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=0
    -c, --create                     Create the sheet if it doesn't exist.  It will use --sheet-name if specified
    -k, --key-values KEY_VALUES      A comma-separated list of key=values which will be made available to the template
    -n, --sheet-name SHEET_NAME      The name of the sheet to apply the template to
    -v, --verbose                    Enable verbose output
    -x, --offset-columns OFFSET      Apply the template offset by OFFSET cells
    -y, --offset-rows OFFSET         Apply the template offset by OFFSET rows
    -h, --help                       Show help information
```

## Usage Examples

```
# apply my_taxes_template.csvpp to an existing Google Sheet with name "Taxes 2022"
$ csv++ --sheet-name "Taxes 2022" --sheet-id "[...]" my_taxes_template.csvpp

# take input from stdin, supply a variable ($$rate = 1) and apply to the "Stocks" spreadsheet
$ cat stocks.csvpp | csv++ -k "rate=1" -n "Stocks" -i "[...]"
```
