# Coding Conventions

While you're free to use whatever coding conventions make sense for you and your existing
spreadsheets, these recommendations provide a good starting point.

## Variables and Functions

Variables and functions should be lowercase and contain only letters, numbers and underscores (`\_`).

### Good
```csvpp
foo := 1
foo_bar := "value"

fn my_long_descriptive_function(a)
  a * foo
---
```

### Bad
```csvpp
FOO := 1            # not preferred, but this will compile
foo-bar := "value"  # this will not compile!

# we prefer snake_case over camelCase but this will compile
fn myLongDescriptiveFunction(a)
  a * foo
---
```

## Module Names

Similar to variables & functions, module names should only contain letters, numbers and underscores
and can be joined with a `/` if it resides in a sub-module.

## Indentation

Indenting with 2 spaces is preferred.
