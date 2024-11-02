# Common Patterns

## Building a Multi-Sheet Workbook

TODO

## Referencing Another Sheet in the Same Workbook

TODO

## Build Systems

Other than being a compiler csv++ does not ship with any kinds of build systems or package
management.  The simplest approach is to use a `Makefile` to build your spreadsheets.  Here is one
that will build all `.csvpp` files in the current directory into CSV and Excel files:

```Makefile
srcs := $(wildcard *.csvpp)
xlsx_files := $(srcs:%.csvpp=%.xlsx)
csv_files := $(srcs:%.csvpp=%.csv)

all: $(xlsx_files) $(csv_files)

%.xlsx: %.csvpp
	csvpp $(CSVPPFLAGS) -o $@ $^

%.csv: %.csvpp
	csvpp $(CSVPPFLAGS) -o $@ $^

clean:
	rm -f *.csv *.csvpo *.xlsx

.PHONY: all clean
```

You can see more examples in the [csv++ examples
repo](https://github.com/patrickomatic/csvpp-examples).
