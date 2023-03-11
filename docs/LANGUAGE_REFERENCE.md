# csv++ Language Reference

## Introduction

csv++ is intended to provide a programming interface for making re-usable spreadsheets 
where the logic can be written and stored in git, but the values used in the spreadsheet
can be provided later.

## Syntax

csv++ allows you to define variables and functions in a code section before the spreadsheet
and then use them in a spreadsheet.  Functions basically act as macros, where variables are
interpolated but they are otherwise inserted into the spreadsheet without being evaluated.
The language is a superset of the formula language used in spreadsheets - you should be able
to do everything you can in a spreadsheet but also define functions and variables.

## Functions

Functions are defined by:

```
def <function-name> ( <arg1>, <arg2>, ... )
```

They can have any number of arguments and will be evaluated in the context of the cell in 
which they are called.

### Examples
```
def minus_one(number) number - 1

def profit(quantity) 
  (celladjacent(A) * $$quantity) - $$fees
```

## Variables

Variables can be defined in the code section by giving a name (a combination of letters, numbers 
and underscores), the expression `:=` and followed with a value:

```
<variable-name> := <expression>
```
### Examples

```
foo := A3
bar := SUM(celladjacent(A), B4)
```
## Built-ins

csv++ comes with a number of builtin functions and variables.  They are all evaluated in the
context of the cell calling them, which is important to note because they can reference cells
around them.

### Variables

* `cellnum` - The (integer) index of the cell being evaluated. Starts at 1
* `cellref` - The current cell (for example A1, B1, etc)
* `rowabove` - The row number of the row above the current one.  For example if the current cell
  is `C5`, it's `4`.
* `rowbelow` - The row number of the row below the current one.  If the current cell is `C5`, it
  would be `6`.
* `rownum` - The (integer) index of the row being evaluated. Starts at 1
* `rowref` - The current row (for example A, B, ZZ, etc)

### Functions

* `cellabove(C)` - Get a reference to a cell on the row above.  For example if the current cell is
  `C5`, calling `cellabove(A)` will yield `A4`.
* `celladjacent(C)` - A reference to a cell on the the same row as the current cell.
* `cellbelow(C)` - Returns a reference to a cell on the row below it.

## Modifiers

Modifiers can change the formatting of a cell or row, apply validation, change alignment, etc. All 
of the normal rules of CSV apply, with the addition that each cell can have modifiers (specified in 
`[[`/`]]` for cells and `![[`/`]]` for rows):

```
foo,[[...]]bar,baz
```

specifying formatting or various other modifiers to the cell.  Additionally a row can start with:

```
![[...]]foo,bar,baz
```

which will apply that modifier to all cells in the row.

### Examples

* Align the second cell left, align the last cell to the center and make it bold and italicized:

```
Date,[[align=left]]Amount,Quantity,[[align=center/format=bold italic]]Price
```

* Underline and center-align an entire row:

```
![[align=center/format=underline]]Date,Amount,Quantity,Price
```

* A header for the first row, then some formulas that repeat for each row for the rest of the spreadsheet:

```
![[align=center/format=bold]]Date,Price,Quantity,Profit
![[expand=1:]],,,"=MULTIPLY(cellref(B), cellref(C))"
```
