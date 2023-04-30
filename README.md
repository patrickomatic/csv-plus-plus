[![Ruby Style Guide](https://img.shields.io/badge/code_style-community-brightgreen.svg)](https://rubystyle.guide)
[![Gem Version](https://badge.fury.io/rb/csv_plus_plus.svg)](https://badge.fury.io/rb/csv_plus_plus)

# csv++

A tool that allows you to programatically author spreadsheets in your favorite text editor and write their results to CSV, Google Sheets, Excel and other spreadsheet formats.  This allows you to write a spreadsheet template, check it into git and push changes out to spreadsheets using typical dev tools.

## Template Language

A `csvpp` file consists of a (optional) code section and a CSV section separated by `---`.  In the code section you can define variables and functions that can be used in the CSV below it.  For example:

###### **`mystocks.csvpp`**
```
fees := 0.50 # my broker charges $0.50 a trade

price := celladjacent(C)
quantity := celladjacent(D)

def profit() (price * quantity) - fees

---
![[format=bold/align=center]]Date   ,Ticker             ,Price  ,Quantity   ,Profit       ,Fees
![[expand]]                         ,[[format=italic]]  ,       ,           ,"=profit()"  ,=fees
```

And can be compiled into a `.xlsx` file by:

```
$ csv++ -n 'My Stock Tracker' -o mystocks.xlsx mystocks.csvpp
```

See the [Language Reference](./docs/LANGUAGE_REFERENCE.md) for a full explanation of features.

## Installing

Just install it via rubygems (homebrew and debian packages are in the works):

`$ gem install csv_plus_plus`

or if you want the very latest changes, clone this repository and run:

`$ rake gem:install`

### [Setting Up Google Sheets](./docs/README_GOOGLE_SHEETS.md)

## Examples

Take a look at the [examples](./examples/) directory for a bunch of example `.csvpp` files.

## CLI Arguments

```
Usage: csv++ [options]
    -h, --help                       Show help information
    -b, --backup                     Create a backup of the spreadsheet before applying changes.
    -c, --create                     Create the sheet if it doesn't exist.  It will use --sheet-name if specified
    -g, --google-sheet-id SHEET_ID   The id of the sheet - you can extract this from the URL: https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=0
    -k, --key-values KEY_VALUES      A comma-separated list of key=values which will be made available to the template
    -n, --sheet-name SHEET_NAME      The name of the sheet to apply the template to
    -o, --output OUTPUT_FILE         The file to write to (must be .csv, .ods, .xls)
    -v, --verbose                    Enable verbose output
    -x, --offset-columns OFFSET      Apply the template offset by OFFSET cells
    -y, --offset-rows OFFSET         Apply the template offset by OFFSET rows
```
