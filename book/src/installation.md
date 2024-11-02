# Installation

Just go to [releases on github](https://github.com/patrickomatic/csv-plus-plus/releases), download
the latest release, unpack it and put it in your $PATH.

## Installing From Source

To install from source check out the [csv++ repository](https://github.com/patrickomatic/csv-plus-plus) 
and run:

```bash
cargo install --path .
```

This assumes you've [installed Rust](https://www.rust-lang.org/tools/install)

## Google Sheets Setup

To publish to Google Sheets you will need to authenticate using the `gcloud` CLI tools. First you 
need to [install the gcloud CLI](https://cloud.google.com/sdk/docs/install) and then run:

```sh
$ gcloud init
$ gcloud auth login --enable-gdrive-access --update-adc
```
