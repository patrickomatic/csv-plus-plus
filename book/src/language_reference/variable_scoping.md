# Variable Scoping

The variable scoping semantics are pretty unique because every function call is evaluated relative
to the cell where it is used.  As you've seen so far you can use `[[var=...]]` to bind a variable
name to a given cell.  As an example of scoping semantics we'll use this csv++ template:

```csvpp
foo_from_code_section := 42
---
[[var=bar_outside_fill]]    ,,,,,
![[fill=2]]bar              , \
  [[var=bar_in_fill]]       , \
  =bar_in_fill              , \
  =bar_outside_fill         , \
  =foo_from_code_section    ,
```

which will compile to:

```csv
     ,     ,     ,     ,
bar  ,     , =B2 , =A1 , =42
bar  ,     , =B3 , =A1 , =42
```

Breaking this down:

* `foo_from_code_section` - Is always `42` no matter where it is used.
* `bar_in_fill` - Since it is defined within an `![[fill]]`, it's value depends on the final
  row, which will be `B2` or `B3`
* `bar_outside_fill` - Will always be `A1`, pointing to the cell where it was defined.  There
  is no relative aspect to it since it's not defined in an `fill`.
