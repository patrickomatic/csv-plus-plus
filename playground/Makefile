CSVCC := csvpp

src_files := $(wildcard *.csvpp)

excel_files := $(src_files:%.csvpp=%.xlsx)
csv_files := $(src_files:%.csvpp=%.csv)

all: csvpp_install $(excel_files) $(csv_files)

.PHONY: all_features_gs
all_features_gs: all_features.csvpp
	$(CSVCC) -g "all features" 

%.csv: %.csvpp
	$(CSVCC) -o $@ $<

%.xlsx: %.csvpp
	$(CSVCC) -o $@ $<

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
