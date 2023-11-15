# Why Rust?

There were several factors deciding to use [Rust](https://www.rust-lang.org/) to write the csv++ 
compiler:

* Fast
* Fearless concurrency.  Compiling a spreadsheet cell-by-cell can take advantage of concurrency,
  so we needed a language that can support it.
* Can build for native compilation targets. We want to be able to distribute the compiler with
  zero or minimal dependencies (preferably no runtime) and compile a static binary for each
  target OS.
* Spreadsheet library support.  We need to be able to write Excel files and to the Google Sheets
  API which means we need a language that has libraries to support that.
