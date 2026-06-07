# Installation

## Install Prebuilt Binaries

### macOS / Linux

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/patrickomatic/csv-plus-plus/releases/latest/download/csvpp-installer.sh | sh
```

### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/patrickomatic/csv-plus-plus/releases/latest/download/csvpp-installer.ps1 | iex"
```

These installers download the right release asset for your platform and place `csvpp` in your
PATH.

## Installing From Source

To install from source check out the [csv++ repository](https://github.com/patrickomatic/csv-plus-plus) 
and run:

```bash
cargo install csvpp
```

This assumes you've [installed Rust](https://www.rust-lang.org/tools/install). If you prefer to
build from a local checkout instead, run `cargo install --path .`.

## Google Sheets Setup

To publish to Google Sheets you will need to authenticate using the `gcloud` CLI tools. First you 
need to [install the gcloud CLI](https://cloud.google.com/sdk/docs/install) and then run:

```sh
$ gcloud init
$ gcloud auth login --enable-gdrive-access --update-adc
```
