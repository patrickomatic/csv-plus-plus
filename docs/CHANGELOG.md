## v0.5.2

### Features

* Eval rows in parallel

## v0.5.1

### Features

* Generate and use `csvpo` cache files (increase compilation speed)
* Ugprade `umya-spreadsheet` to v1.1.1
* Deprecate `Error::ObjectCodeError`

### Bugfixes

* Fix leaking of namespaces from dependencies making their way into the dependent.  Only exported
  symbols should have been propagated
* Fix evaluation of variables dependent on variables from another file
* Fix lexing of the `fn` keyword in places like `fn fn_name` where the "fn" part is repeated twice

## v0.5.0

### Features

* `use` statement for importing code from other files
* Using object code files (.csvpo) during compilation
* Allow for `.` characters in a function name
* Support various levels of verbosity by repeating -v (i.e., -vvvv)
* Improved error messages for syntax errors in cells
* Use a proper logger (env\_logger) and tune output according to the -v[vvv] flag
* Code coverage reporting
* Tooling to run benchmarks

### Bugfixes

* Fix unfriendly error when calling a function with the wrong number of arguments

### **Breaking (Language) Changes**

* All builtin variables & functions are removed.  All of the functionality provided by them could
  be done with native spreadsheet functions anyway.  And we're adding support for module loading 
  which means they could just be implemented as a lib instead.
* Rename `![[expand]]` to `![[fill]]` and all related code references
* Rename `[[format]]`  to `[[text]]` 
* Rename `Template` to `Module`
* Rename `CodeSection` to `Scope`
* Rename `Error::ObjectWriteError` to `Error::ObjectCodeError`
* `Template.write_object_file` made crate-private
* `Error::EvalError` changed
