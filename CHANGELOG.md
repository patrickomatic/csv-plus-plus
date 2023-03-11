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
