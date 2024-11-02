# Cell & Row Options

## Options

You can change the formatting of a cell or row, bind cells to variables, apply validation, change
alignment, etc. All of the normal rules of CSV apply with the addition that each cell can have 
options specified in `[[`/`]]` for cells and `![[`/`]]` for rows:

```csvpp
foo,[[...]]bar,baz
```

specifying formatting or various other options to the cell.  Additionally a row can start with:

```csvpp
![[...]]foo,bar,baz
```

which will apply to all cells in the row.


### All Options

#### border = all | top | bottom | left | right
Sets a border on the given side (or all four sides if `all`). Can be repeated multiple times to set 
multiple borders.

##### Alias
`b = a | t | b | l | r`


#### bordercolor = HEX\_COLOR
The color of the border, where `HEX_COLOR` is a 6-character hex color code.

##### Alias
`bc = HEX_COLOR`


#### borderstyle = dashed | dotted | double | solid | solid\_medium | solid\_thick
The style of the border. `solid` is the default if a border is set and it is not specified.

##### Alias
`bs = dash | dot | dbl | 1 | 2 | 3`


#### color = HEX\_COLOR
The color of the cell, where `HEX_COLOR` is a 6-character hex color code.

##### Alias
`c = HEX_COLOR`


#### fill
#### fill = AMOUNT
Duplicate the row `AMOUNT` times.  If `AMOUNT` is not supplied, the row will be repeated for the rest of the sheet.

##### Alias
`f = AMOUNT` (optional)


#### fontcolor = HEX\_COLOR
The color of the font, where `HEX_COLOR` is a 6-character hex color code.

##### Alias
`fc = HEX_COLOR`


#### fontfamily = Arial | Helvetica | ...
The font family to use.  It must be a valid font, compatible with your target spreadsheet

##### Alias
`ff = FONT_FAMILY`


#### fontsize = INTEGER
The font size to use, as a whole number.

##### Alias
`fs = INTEGER`


#### halign = left | center | right
The horizontal alignment.

##### Alias
`ha = l | c | r`


#### lock
Prevent the cell or row from being modified.

##### Alias
`l`


#### note = STRING
A note to associate with the cell. The `STRING` should be quoted with single quotes and you can escape quotes like: `note='You\\'re taking a note'`

##### Alias
`n = STRING`


#### numberformat = currency | date | datetime | number | percent | text | time | scientific
The number format to apply to the cell.

##### Alias
`nf = c | d | dt | n | p | text | t | s`


#### text = bold | italic | strikethrough | underline
Applies the given format. Can be repeated multiple times to set multiple formats.

##### Alias
`t = b | i | s | u`


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

##### Alias
`va = b | c | t`


#### var = VARIABLE\_ID
Bind a variable (specified by `VARIABLE_ID`) to reference this cell.

##### Alias
`v = VARIABLE_ID`


### Examples

* Align the second cell left, align the last cell to the center and make it bold and italicized:

```csvpp
Date,[[ha=l]]Amount,Quantity,[[ha=c t=b t=i]]Price
```

* Underline and center-align an entire row:

```csvpp
![[ha=c t=u]]Date,Amount,Quantity,Price
```

* A header for the first row, then some formulas that repeat for each row for the rest of the 
spreadsheet:

```csvpp
![[ha=c t=b]]Date   ,Price          ,Quantity           ,Profit
![[fill]]           ,[[var=price]]  ,[[var=quantity]]   ,"=MULTIPLY(price, quantity)"
```
