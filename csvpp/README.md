# csv++

At the most basic this is a tool that can convert CSV to Excel, Google Sheets or ODF.  Taken 
further this is a superset of CSV as a programming language.  You can specify formatting in the
spreadsheet:

```csvpp
[[format=bold/format=underline]]foo,[[fontsize=20]]bar,baz
```

and using shorthand:

```csvpp
[[f=b/f=u]]foo,[[fs=20]]bar,baz
```

You can also extract re-usable variables and functions by making a code section at the top, separated 
from the cells by a `---`

```csvpp
# you can define variables with `:=`
foo := 42

# functions look like this have a single expression as their body
fn bar(a, b)
  a + b

---
foo,=bar(10, 20),=foo
some,other,values
```

