# csv++

csv++ is an extension of [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) which allows
you to author spreadsheets in a text file then compile that to your target spreadsheet (Excel, 
Google Sheets or even back to CSV).

Since csv++ is a superset of CSV any CSV document is valid:

```csvpp
Sum,Column 1,Column 2,
=5*SUM(B2,C2),,,
```

However you can extract reusable variables and functions by making a code section at the top, 
separated from the cells with a `---`.  Here's the same spreadsheet but extracted out into
variables and functions:

```csvpp
# you can define variables with `:=`
multiplier := 5

# functions look like this and have a single expression as their body
fn my_fn(a, b)
  multiplier * SUM(a, b)

---
Sum             , Column 1, Column 2,
=my_fn(B2, C2)  ,         ,         ,
```

One more useful feature is the ability to bind variables to a cell.  You can use the `[[`/`]]`
modifier syntax on the cell you want to bind it on.

```csvpp
fn my_complex_fn(a, b)
  a * a + SQRT(b)

---
Complex function    , Column 1  , Column 2 ,
=my_complex_fn(a, b), [[var=a]] , [[var=b]],
```

## Expands

Another useful feature is to define a range of rows which expand out (either infinitely or by a
finite amount) in the compiled spreadsheet.  To specify one you use the row modifier syntax
which is similar to above, you just prefix it with `!`: `![[`/`]]`.

```csvpp
Product Name   , Quantity          , Price per Unit  , =SUM(D2:D12)
![[expand=10]] , [[var=quantity]]  , [[var=price]]   , =quantity * price
```

This will expand the second row and repeat it 10 times in the final spreadsheet.  If you wanted 
it to be repeated until the end of the spreadsheet just leave off the `=10` and specify it as 
`![[expand]]`.

## Variable Scoping

TODO

## Builtin Functions & Variables

TODO

## Formatting

You can also specify basic cell formatting which will either apply for the entire row or just
for individual cells.  To apply formatting to individual cells use the `[[`/`]]` syntax:

```csvpp
[[format=bold/format=underline]]foo,[[fontsize=20]]bar,baz
```

and here is the same thing using short-hand:

```csvpp
[[f=b/f=u]]foo,[[fs=20]]bar,baz,
```

To apply formatting to the entire row you can use `![[`/`]]` at the beginning of the row

```csvpp
![[f=b/f=u]]foo,bar,baz,
```

For a full list of formatting features, take a look at the [language reference](docs/LANGUAGE_REFERENCE.md)

### Additional Reading

* [Changelog](docs/CHANGELOG.md)
* [Language Reference](docs/LANGUAGE_REFERENCE.md)
* [Target Spreadsheet Feature Support Matrix](docs/feature_matrix.csvpp)
