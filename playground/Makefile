# load .env if it's set. (see .env.example as an example to get started)
ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# CSVCC := csvpp
CSVCC := cargo run --

src_files := $(wildcard *.csvpp)

excel_files := $(src_files:%.csvpp=%.xlsx)
csv_files := $(src_files:%.csvpp=%.csv)

all: $(excel_files) $(csv_files)

.PHONY: all_features_gs
test_gs: test.csvpp
	$(CSVCC) -vv --sheet-name "test" -g $(GOOGLE_SHEETS_TEST_ID) $<

%.csv: %.csvpp
	$(CSVCC) -vvv -o $@ $<

%.xlsx: %.csvpp
	$(CSVCC) -vvv -o $@ $<

.PHONY: open
open: all
	open ./all_features.xlsx

.PHONY: csvpp_install
csvpp_install:
	cd ../ && cargo install --path .

.PHONY: clean
clean:
	rm -f *.xlsx *.csv *.csvpo

.PHONY: kill_excel
kill_excel:
	@killall 'Microsoft Excel' || true
