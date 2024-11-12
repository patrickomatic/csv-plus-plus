## v0.8.0

### **Breaking Changes**

* Added enum variant `Error::CsvParseError`
* Renamed `Options` to `Config`

### Features

* Slashes are no longer needed to separate cell options
    - `[[text=bold halign=left]]` is now equivalent to `[[text=bold/halign=left]]`.
* Support for multi-line cells
* [csv++ User Guide](https://patrickomatic.github.io/csv-plus-plus/)
* Switch to custom CSV parser (`csvp`)
* Switch from CBOR to bincode for object file serialization

### Deprecated Features

* You should no longer use `/` to separate cell options

### Dependency Updates


## v0.7.0

### Features

* Build a statically-linked `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` for people
  who are on a system with an outdated glibc
* Quoting rules for single and double quoted strings have changed to align with OpenFormula and how
  popular spreadsheet programs do it.  Rather than using a backslash like `"a \"quote\""` or 
  `'a \'quote\''` you just double the character.  So `"a ""quote"""` and `'a ''quote'''`.
* Builds for more platforms:
  - aarch64-unknown-linux-musl
  - i686-unknown-linux-gnu
  - i686-unknown-linux-musl
  - i686-pc-windows-gnu
  - x86\_64-unknown-linux-musl
* Switch from `a1_notation` crate to `a1`

### Bugfixes

* Fix a bug causing double-quoted strings (i.e, `""foo""`)

### **Breaking Changes**

* Double-quoted strings now use double-quote (`""`) to escape a single-quote.  Previously it used 
  `\"` to escape.
