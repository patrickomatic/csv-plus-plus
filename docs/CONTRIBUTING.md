# Contributing

Just open a PR!  Or feel free to [create an issue](https://github.com/patrickomatic/csv-plus-plus/issues)
if you would like feedback on your idea before working on it.

## Design Philosophies

Some design philosophies that you should abide by if you want to contribute source code:

### Versioning

* Make things private by default.  We want to reduce incrementing versions for trivial changes 
and to do that we should expose as little as possible.

  - **Things to make public**: Anything needed by `src/lib.rs` or otherwise would be necessary
  for someone to compile a template from another Rust program.

  - **Things to make private/pub(crate)**: Try to make things private by default, but in 
  particular the parser and target-specific (excel, google sheets, etc) code should not be 
  exposed.  Ideally we should be able to change these without churning the version numbers.
    
### Dependencies

* As few dependencies as possible.  If you don't absolutely need the dependency, it's better
*not* to include it.

* Target the lowest version of the dependency you need.  Ideally we want to be as flexible as
possible if someone includes the csv++ source code, so that means not targetting high 
dependency versions unless necessary.

* Target as few features as possible from the dependency.  If the dependency splits it's 
functionality into multiple features than only include what you need.
