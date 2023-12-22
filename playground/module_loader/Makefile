CSVCC := cargo run --
# CSVCC := csv++
CFLAGS := -vvvv 

all: main.csv main.xlsx

%.csv: %.csvpp
	$(CSVCC) $(CFLAGS) -o $@ $<

%.xlsx: %.csvpp
	$(CSVCC) $(CFLAGS) -o $@ $<

.PHONY: clean
clean:
	rm -f *.xlsx *.csv *.csvpo

