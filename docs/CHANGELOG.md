## v0.7.0 (upcoming)

### Features

* Allow for a single infinite fill but any number of finite fills before and after it
* Support `%` postfix operator
* Support `=` comparison operator
* Support `+` prefix operator
* Upgrade `env_logger` to v0.11.1

### Bugfixes

* Fix a bug where the spreadsheet was not getting re-compiled if a .csvpo file existed
* Fix `</<=/>/>=` lexing
* Don't allow (throw an error) if there are multiple infinite fills

### Breaking Changes

* Operator precedence has changed to match the [OpenFormula spec](https://docs.oasis-open.org/office/v1.2/os/OpenDocument-v1.2-os-part2.html#__RefHeading__1017940_715980110)

## v0.6.0

### Features

* Eval rows in parallel
* Support for backing up on Google Sheets
* Dates, times and datetimes are now tz-unaware

### Bugfixes

* Fix fills being evaluated every time they're loaded from a .csvpo cache (and blowing up the 
  resulting spreadsheet)
* The main module will always use it's csvpo cache when possible
* Fixes a potential overflow when calculating fills past row 1000

### **Breaking Changes**

* `DateTime` no longer supports a variant with fixed TZ offset
* Various functions on `Fill` made crate-private
* Removed `Error::ObjectCodeError` variant
