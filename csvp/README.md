# `csvp` (aka `csv+`)

An "enhanced" CSV parser for use internally by `csv++`.  The reasons we need a custom CSV parser and
the [csv crate](https://docs.rs/csv/latest/csv/) will no longer suffice:

* Support for wrapping a cell across multiple lines.
* Comments (a row starting with `#`).
* Contextual information with each cell - in addition to just the value it parses to we need to know
  enough info about what line certain values happened on, in order to show proper highlighting and
  error messaging.
