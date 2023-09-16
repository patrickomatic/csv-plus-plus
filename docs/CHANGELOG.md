## main

### Bugfixes

* Fix cellabove/cellbelow/celladjacent to all take columns (A/B/C/etc)

## v0.3.0

Complete rewrite in Rust, which also includes:

* shorthand notation for modifiers
* much speed
* a re-usable A1 library (a1\_notation)
* better error messages since I've hand-written the parser

## v0.2.1

- Add a `-s`/`--safe` flag which changes the merge strategy to not overwrite existing values.  If the spreadsheet being written to has values where the csvpp template wants to write, they will be overwritten otherwise.

## v0.2.0

### **Breaking Changes**

- Removal of the $$ operator - to dereference variables you can just reference them by name and they will be resolved if they are defined.  Otherwise they will be left alone in the output

### Non-breaking Changes

- Excel: fix the merging of existing values 
- CSV: fix the merging of existing values 
- Support merging in values from CSV (previously it would ignore/overwrite them)
- Allow for more generous spacing in the csv section (and reflect this in the examples)
- More type coverage

## v0.1.3

- Proper scoping of variables defined within an expand modifier
- Types via Sorbet
- Fix formula insertion on Excel
- Fix modifier string quoting
- Fix broken Yard doc generation
- Fix: multiple modifiers on the same row weren't being handled 

## v0.1.2

- var=... modifier which allows binding a variable to a cell
- Improved error handling and messages
- Moving in a direction that allows for the context-dependent aspects of modifiers
- Fixes a bug with creating a new excel spreadsheet
- Docs & tests

## v0.1.1

- Better support for the various infix operators (+,-,/,*,^,%,=,<,etc)
  * Previously we were converting them to their prefix equivalent (multiply, minus, concat, etc) but excel doesn't support most of those.  So we keep them infix
  * Didn't support some infix operators (^, %, </>/<=/>=/<>)
  * Proper support for operator precedence
- When in verbose mode, print a summary of compiled functions and variables
- docs & tests

## v0.1.0

- revamp of builtin functions
- docs & tests

## v0.0.5

- Support the --backup/-b option
- bin/csvpp (which does the same thing as bin/csv++ but will work better on other filesystems)
- Fix links in gemspec (which end up on rubygems.org)
- docs & tests

## v0.0.4

- Excel support

## v0.0.3

- Fix the gem package to include the bin/ file

## v0.0.2

- First publish to rubygems.org
