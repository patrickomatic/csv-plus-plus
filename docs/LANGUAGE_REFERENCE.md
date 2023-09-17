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

```csvpp
fn <function-name> ( <arg1>, <arg2>, ... )
```

They can have any number of arguments and will be evaluated in the context of the cell in 
which they are called.

#### Examples
```csvpp
fn minus_one(number) number - 1

fn profit(quantity) 
  (celladjacent(A) * quantity) - fees
```

## Variables

Variables can be defined in the code section by giving a name (a combination of letters, numbers 
and underscores), the expression `:=` and followed with a value:

```
<variable-name> := <expression>
```
#### Examples

```csvpp
foo := A3
bar := SUM(celladjacent(A), foo)
```

To reference a variable you just use it by it's name - there is no special operator to dereference.  

## Cell References

You can use A1-style cell references in the code section and within function definitions.  Since the
syntax for variable references and for cell references are overlapping (for example `ABC` is both a 
valid cell reference and variable reference), csv++ will only interpolate the variable if it is
defined.  Otherwise it will be left alone and treated as a cell reference.

## Built-ins

csv++ comes with a number of builtin functions and variables.  They are all evaluated in the
context of the cell calling them, which is important to note because they can reference cells
around the calling cell.

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

## Scope Semantics

At this point it's worth mentioning that csv++ has pretty unique variable and function scoping 
semantics - all functions and variables are dynamically evaluated in the context of the cell in which 
they are used.  For example you can define a variable (or function) in the code section which 
references cells relative to the place where they are used:

```csvpp
# interest_rate is bound to cell B1 via the var= modifier below.
# cell_adjacent(A) gives us a reference to the "Amount" value for each row
interest_on_amount := interest_rate * cell_adjacent(A)
---
Interest Rate:,[[var=interest_rate]]0.05
[[format=bold]Amount,Amount with interest
50000,=interest_on_amount
100000,=interest_on_amount
500000,=interest_on_amount
```

in the above example, `interest_on_amount` will be evaluated per the different amount on each
row.  This gets interesting when combined with the `![[expand=]]` directive:

```
# TODO: make an example
```

## Modifiers

Modifiers can change the formatting of a cell or row, bind cells to variables, apply validation, 
change alignment, etc. All of the normal rules of CSV apply, with the addition that each cell can 
have modifiers (specified in `[[`/`]]` for cells and `![[`/`]]` for rows):

```
foo,[[...]]bar,baz
```

specifying formatting or various other modifiers to the cell.  Additionally a row can start with:

```
![[...]]foo,bar,baz
```

which will apply that modifier to all cells in the row.

### All Modifiers

#### border = all | top | bottom | left | right
Sets a border on the given side (or all four sides if `all`). Can be repeated multiple times to set multiple borders.

> ##### Alias: `b = a | t | b | l | r`


#### bordercolor = HEX_COLOR
The color of the border, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `bc = HEX_COLOR`


#### borderstyle = dashed | dotted | double | solid | solid_medium | solid_thick
The style of the border. `solid` is the default if a border is set and it is not specified.

> ##### Alias `bs = dash | dot | dbl | 1 | 2 | 3`


#### color = HEX\_COLOR
The color of the cell, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `c = HEX_COLOR`


#### expand
#### expand = AMOUNT
Duplicate the row `AMOUNT` times.  If `AMOUNT` is not supplied, the row will be repeated for the rest of the sheet.

> ##### Alias `e = AMOUNT` (optional)


#### fontcolor = HEX\_COLOR
The color of the font, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `fc = HEX_COLOR`


#### fontfamily = Arial | Helvetica | ...
The font family to use.  It must be a valid font, compatible with your target spreadsheet

> ##### Alias `ff = FONT_FAMILY`


#### fontsize = INTEGER
The font size to use, as a whole number.

> ##### Alias `fs = INTEGER`


#### format = bold | italic | strikethrough | underline
Applies the given format. Can be repeated multiple times to set multiple formats.

> ##### Alias `f = b | i | s | u`


#### halign = left | center | right
The horizontal alignment.

> ##### Alias `ha = l | c | r`


#### lock
Prevent the cell or row from being modified.

> ##### Alias `l`


#### note = STRING
A note to associate with the cell. The `STRING` should be quoted with single quotes and you can escape quotes like: `note='You\\'re taking a note'`

> ##### Alias `n = STRING`


#### numberformat = currency | date | datetime | number | percent | text | time | scientific
The number format to apply to the cell.

> ##### Alias `nf = c | d | dt | n | p | text | t | s`


#### validate
- TODO

#### valign = bottom | center | top
The vertical alignment.

> ##### Alias `va = b | c | t`


#### var = VARIABLE\_ID
Bind a variable (specified by `VARIABLE_ID`) to reference this cell. TODO

> ##### Alias `v = VARIABLE_ID`


#### Examples

* Align the second cell left, align the last cell to the center and make it bold and italicized:

```csvpp
Date,[[align=left]]Amount,Quantity,[[align=center/format=bold italic]]Price
```

* Underline and center-align an entire row:

```csvpp
![[align=center/format=underline]]Date,Amount,Quantity,Price
```

* A header for the first row, then some formulas that repeat for each row for the rest of the spreadsheet:

```csvpp
![[align=center/format=bold]]Date,Price,Quantity,Profit
![[expand=1:]],,,"=MULTIPLY(cellref(B), cellref(C))"
```
