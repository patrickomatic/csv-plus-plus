# Using the CLI

The most common use of the CLI is just to compile a csv++ source file to you desired target.  The
target is inferred from the CLI options.  Given that you have your source code in `my_sheet.csvpp`:

## Building to Excel

```sh
$ csvpp -o my_sheet.xlsx my_sheet.csvpp
```

## Building to Google Sheets

```sh
$ csvpp --google-sheet-id "the-google-sheet-id" my_sheet.csvpp
```

This assumes that you have [set up Google Sheets access](./installation.md#google-sheets-setup).

## Building to CSV

While you will lose all formatting options, you can also compile back to CSV:

```sh
$ csvpp -o my_sheet.csv my_sheet.csvpp
```

## Increasing Verbosity

If you're having trouble debugging an issue, you can increase the verbosity of the output by adding
the `-vvvv` option.
