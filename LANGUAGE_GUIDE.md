# csv++ Language Reference

## Introduction

csv++ is intended to provide a programming interface for making re-usable spreadsheets 
where the logic can be written and stored in git, but the values used in the spreadsheet
can be provided later.  

## Syntax

csv++ allows you to define variables and functions in a code section before the spreadsheet
and then use them in a spreadsheet.  Functions basically act as macros, where variables are
interpolated but they are otherwise inserted into the spreadsheet without their 

## Functions

## Variables

## Built-ins

### Variables

* `cellnum` - The (integer) index of the cell being evaluated. Starts at 1
* `cellref` - The current cell (for example A1, B1, etc)
* `rownum` - The (integer) index of the row being evaluated. Starts at 1
* `rowref` - The current row (for example A, B, ZZ, etc)

### Functions

* `cellabove`
* `cellbelow`
* `rowabove`
* `rowbelow`
