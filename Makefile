BIN_DIR := ~/bin
RELEASE_DIR := release

VERSION := $(shell git describe --tags --candidates 1)

TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu \
					 x86_64-apple-darwin aarch64-apple-darwin \
					 x86_64-pc-windows-gnu

RELEASE_DIRS := $(addprefix $(RELEASE_DIR)/csvpp-$(VERSION)-, $(TARGETS))
RELEASE_FILES := $(RELEASE_DIRS:=.tar.gz)
RELEASE_FILE_SIGS := $(RELEASE_FILES:=.asc)

.PHONY: all
all: $(RELEASE_FILES) $(RELEASE_FILE_SIGS)

$(RELEASE_DIR)/csvpp-$(VERSION)-%.tar.gz: RELEASE_DIR=$(@:.tar.gz=)

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
$(RELEASE_DIR)/csvpp-$(VERSION)-x86_64-pc-windows-gnu.tar.gz:
	cross build --release --target x86_64-pc-windows-gnu

	mkdir -p $(RELEASE_DIR)
	cp LICENSE.txt README.md docs/*.md $(RELEASE_DIR)
	cp target/x86_64-pc-windows-gnu/release/csvpp.exe $(RELEASE_DIR)

	cd release && tar -czf $(shell basename $@) $(shell basename $(RELEASE_DIR))

# the other platforms don't need a file extension so they can all be packaged the same
$(RELEASE_DIR)/csvpp-$(VERSION)-%.tar.gz:
	cross build --release --target $*

	mkdir -p $(RELEASE_DIR)
	cp LICENSE.txt README.md docs/*.md $(RELEASE_DIR)
	cp target/$*/release/csvpp $(RELEASE_DIR)
	cp target/$*/release/csvpp $(RELEASE_DIR)/csv++

	cd release && tar -czf $(shell basename $@) $(shell basename $(RELEASE_DIR))

%.tar.gz.asc: %.tar.gz
	cd $(RELEASE_DIR) && gpg --detach-sign --armor $(shell basename $^)

.PHONY: clean
clean:
	rm -rf $(RELEASE_FILES) $(RELEASE_FILE_SIGS) $(RELEASE_DIRS)
