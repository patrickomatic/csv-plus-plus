# File Structure

Each csv++ source code file contains two distinct parts: an optional code section and a CSV section.

## Code Section

The code section is where variables and functions can be defined and subsequently used in the CSV
section.  It is terminated by the `---` separator and anything following that will be treated as the
CSV section. 

## Functions

Functions basically act as macros, where variables are interpolated but they are 
otherwise inserted into the spreadsheet without being evaluated. The language is a superset of the 
formula language used in spreadsheets - you should be able to do everything you can in a spreadsheet
but also define functions and variables.

Functions are defined by:

```csvpp
fn <function-name> ( <arg1>, <arg2>, ... )
```

They can have any number of arguments and will be evaluated in the context of the cell in 
which they are called.

#### Examples

```csvpp
fn fees(quantity) 
  quantity * 0.10

fn profit(price, quantity) 
  (price * quantity) - fees(quantity)
---
```

## Variables

Variables can be defined in the code section by giving a name (a combination of letters, numbers 
and underscores), the expression `:=` and followed with a value:

```
<variable-name> := <expression>
```

To reference a variable you just use it by it's name - there is no special operator to dereference.  

#### Examples

```csvpp
foo := 42
bar := foo * 2
---
=foo,=bar,
```

will evaluate to:

```
=42,=(42 * 2),
```

## Cell References

You can use A1-style cell references in the code section and within function definitions.  Since the
syntax for variable references and for cell references are overlapping (for example `ABC` is both a 
valid cell reference and variable reference), csv++ will only interpolate the variable if it is
defined.  Otherwise it will be left alone and treated as a cell reference.


# CSV Section

The CSV section defines the stucture of your spreadsheet and makes use of the functions and 
variables defined above. 

## Variables

An important feature of the CSV section is being able to assign variables to cells and reference
them elsewhere.  You do this using the `[[var=<var-name>]]` syntax:

```csvpp
profit := income - fee
---
[[var=fee]],    [[var=income]],     =profit,
```

which will evaluate to:

```
,,=(B1 - A1)
```

### Row Variables

You can also assign a variable to reference an entire row using the row option syntax (`![[`/`]]`):

```csvpp
![[var=row_1]],,,
```
## Fills

A common feature of spreadsheets is to drag the bottom right corner and that formula will be applied
over a range of rows.  The `![[fill=<number-of-rows>]]` provides similar functionality.  

```csvpp
![[fill=3]]foo,bar,baz,
```

evaluates to:

```
foo,bar,baz,
foo,bar,baz,
foo,bar,baz,
```

which is not very helpful but becomes interesting when you combine it with variables.

```csvpp
# fees are a fixed $2.00 per trade
fees := 2

fn profit(quantity, price_each)
  (price_each * quantity) - fees

---
![[fill=3]][[var=number_of_shares]],    [[var=price]],  "=profit(number_of_shares, price)",
```

becomes:

```csvpp
,,"=(B1 * A1) - 2",
,,"=(B2 * A2) - 2",
,,"=(B3 * A3) - 2",
```

A full list of cell & row options will be discussed in the [Cell & Row Options section](./cell\_and\_row\_options.md).

## Comments

Any line in the CSV section will be treated as a comment and ignored if it starts with a `#`:

```csvpp
foo,bar,baz,
# this is a comment
foo1,bar1,baz1,
```

## Multi-line Cells

When using lots of cell options the lines can get pretty long.  To help with this you can split a
single spreadsheet row over multiple lines using the `\\` syntax like so:

```csvpp
---
![[halign=center \
  valign=top \
  text=bold \
  text=underline \
  ]] This,  Is, A,  Header, Row,
[[var=one]] , \
  [[var=two]] , \
  [[var=three]] , \
  [[var=four]] ,
```

which is equivalent to:

```csvpp
---
![[halign=center valign=top text=bold text=underline]] This,  Is, A,  Header, Row,
[[var=one]] , [[var=two]] , [[var=three]] , [[var=four]] ,
```
