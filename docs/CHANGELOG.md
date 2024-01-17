## v0.6.1 (upcoming)

### Features

* Allow for a single infinite fill but any number of finite fills before and after it
* Support `=` comparison operator

### Bugfixes

* Don't allow (throw an error) if there are multiple infinite fills

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
