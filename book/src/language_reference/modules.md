# Modules

Modules are the basic building block of making reusable spreadsheets.  Module names are derived from
the name of the file - so for example if you have `foo.csvpp`, it's module name is `foo`.  

## Requiring Modules

If you want to re-use variables or functions from another file, you can just import that module:

##### `foo.csvpp`
```csvpp
my_constant := 42
---
```

##### `bar.csvpp`
```csvpp
use foo

fn my_fn(a) a * my_constant
---
[[var=a]],=my_fn(a),
```

Currently when importing a module, all of the variables 

## The Main Module

Similar to C/Java/Rust etc, csv++ requires that there is a "main" module.  This is generally
implicit as the file being compiled without a need to specify it.  Only the spreadsheet section of
the main module will be built - for any required/non-main modules the spreadsheet section will be
ignored and only the code section brought into scope.
