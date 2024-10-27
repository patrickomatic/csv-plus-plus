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


## Scope Semantics

At this point it's worth mentioning that csv++ has pretty unique variable and function scoping 
semantics - all functions and variables are dynamically evaluated in the context of the cell in which 
they are used.  For example you can define a variable (or function) in the code section which 
references cells relative to the place where they are used:

```csvpp
# interest_rate is bound to cell B1 via the var= option below.
# cell_adjacent(A) gives us a reference to the "Amount" value for each row
interest_on_amount := interest_rate * cell_adjacent(A)
---
Interest Rate:,[[var=interest_rate]]0.05
[[text=bold]Amount,Amount with interest
50000,=interest_on_amount
100000,=interest_on_amount
500000,=interest_on_amount
```

in the above example, `interest_on_amount` will be evaluated per the different amount on each
row.  This gets interesting when combined with the `![[fill=]]` directive:

```
# TODO: make an example
```


## Options

You can change the formatting of a cell or row, bind cells to variables, apply validation, change
alignment, etc. All of the normal rules of CSV apply with the addition that each cell can have 
options specified in `[[`/`]]` for cells and `![[`/`]]` for rows:

```
foo,[[...]]bar,baz
```

specifying formatting or various other options to the cell.  Additionally a row can start with:

```
![[...]]foo,bar,baz
```

which will apply to all cells in the row.


### All Options

<details>
  <summary><h4>border = all | top | bottom | left | right</h4></summary>

  <p>Sets a border on the given side (or all four sides if `all`). Can be repeated multiple times 
  to set multiple borders.</p>

  <h5>Alias:</h5>
  <blockquote>b = a | t | b | l | r</blockquote>
</details>


#### bordercolor = HEX\_COLOR
The color of the border, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `bc = HEX_COLOR`


#### borderstyle = dashed | dotted | double | solid | solid_medium | solid_thick
The style of the border. `solid` is the default if a border is set and it is not specified.

> ##### Alias `bs = dash | dot | dbl | 1 | 2 | 3`


#### color = HEX\_COLOR
The color of the cell, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `c = HEX_COLOR`


#### fill
#### fill = AMOUNT
Duplicate the row `AMOUNT` times.  If `AMOUNT` is not supplied, the row will be repeated for the rest of the sheet.

> ##### Alias `f = AMOUNT` (optional)


#### fontcolor = HEX\_COLOR
The color of the font, where `HEX_COLOR` is a 6-character hex color code.

> ##### Alias `fc = HEX_COLOR`


#### fontfamily = Arial | Helvetica | ...
The font family to use.  It must be a valid font, compatible with your target spreadsheet

> ##### Alias `ff = FONT_FAMILY`


#### fontsize = INTEGER
The font size to use, as a whole number.

> ##### Alias `fs = INTEGER`


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


#### text = bold | italic | strikethrough | underline
Applies the given format. Can be repeated multiple times to set multiple formats.

> ##### Alias `t = b | i | s | u`


#### validate
Validations that can be applied to the data in the cell.

* `validate=custom(FORMULA)` (alias: `validate=c(FORMULA)`)
* `validate=date_after(DATE)` (alias: `validate=date_gt(DATE)`)
* `validate=date_before(DATE)` (alias: `validate=date_lt(DATE)`)
* `validate=date_between(DATE DATE)` (alias: `validate=date_btwn(DATE DATE)`)
* `validate=date_equal_to(DATE)` (alias: `validate=date_eq(DATE)`)
* `validate=in_list(..)`
* `validate=in_range(A1)`
* `validate=date_is_valid` (alias: `validate=is_date`)
* `validate=is_valid_email` (alias: `validate=is_email`)
* `validate=is_valid_url` (alias: `validate=is_url`)
* `validate=date_not_between(DATE DATE)` (alias: `validate=date_nbtwn(DATE DATE)`)
* `validate=date_on_or_after(DATE)` (alias: `validate=date_gte(DATE)`)
* `validate=date_on_or_before(DATE)` (alias: `validate=date_lte(DATE)`)
* `validate=number_between(NUMBER NUMBER)` (alias: `validate=number_btwn(NUMBER NUMBER)`)
* `validate=number_equal_to(NUMBER)` (alias: `validate=number_eq(NUMBER)`)
* `validate=number_greater_than(NUMBER)` (alias: `validate=number_gt(NUMBER)`)
* `validate=number_greater_than_or_equal_to(NUMBER)` (alias: `validate=number_gte(NUMBER)`)
* `validate=number_less_than(NUMBER)` (alias: `validate=number_lt(NUMBER)`)
* `validate=number_less_than_or_equal_to(NUMBER)` (alias: `validate=number_lte(NUMBER)`)
* `validate=number_not_between(NUMBER NUMBER)` (alias: `validate=number_nbtwn(NUMBER NUMBER)`)
* `validate=number_not_equal_to(NUMBER)` (alias: `validate=number_neq(NUMBER)`)
* `validate=text_contains(TEXT)`
* `validate=text_does_not_contain(TEXT)`
* `validate=text_equal_to(TEXT)` (alias: `text_eq`)


#### valign = bottom | center | top
The vertical alignment.

> ##### Alias `va = b | c | t`


#### var = VARIABLE\_ID
Bind a variable (specified by `VARIABLE_ID`) to reference this cell. TODO

> ##### Alias `v = VARIABLE_ID`


#### Examples

* Align the second cell left, align the last cell to the center and make it bold and italicized:

```csvpp
Date,[[ha=l]]Amount,Quantity,[[ha=c t=b t=i]]Price
```

* Underline and center-align an entire row:

```csvpp
![[ha=c t=u]]Date,Amount,Quantity,Price
```

* A header for the first row, then some formulas that repeat for each row for the rest of the spreadsheet:

```csvpp
![[ha=c t=b]]Date,Price,Quantity,Profit
![[e]],,,"=MULTIPLY(cellref(B), cellref(C))"
```
