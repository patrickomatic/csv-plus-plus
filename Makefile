RELEASE_DIR := release
TARGET_DIR := target

VERSION := $(shell git describe --tags --candidates 1)

WINDOWS_TARGET := x86_64-pc-windows-gnu
TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu \
					 x86_64-apple-darwin aarch64-apple-darwin \
					 $(WINDOWS_TARGET)

# the .tar.gz files and relevant signatures
RELEASE_DIRS := $(addprefix $(RELEASE_DIR)/csvpp-$(VERSION)-, $(TARGETS))
RELEASE_FILES := $(RELEASE_DIRS:=.tar.gz)
RELEASE_FILE_SIGS := $(RELEASE_FILES:=.asc)

DOC_FILES := $(wildcard docs/*.md) LICENSE.txt README.md
DOC_FILES := $(filter-out docs/RELEASE_CHECKLIST.md, $(DOC_FILES))

.PHONY: all
all: $(RELEASE_FILES) $(RELEASE_FILE_SIGS)

.PHONY: clean
clean:
	cargo clean --release
	rm -rf $(RELEASE_FILES) $(RELEASE_FILE_SIGS) $(RELEASE_DIRS)

$(TARGET_DIR)/%/release/csvpp.exe:
$(TARGET_DIR)/%/release/csvpp:
	cross build --release --target $*

$(RELEASE_DIR)/csvpp-$(VERSION)-%.tar.gz: PREP_DIR=$(@:.tar.gz=)

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
$(RELEASE_DIR)/csvpp-$(VERSION)-%/csvpp.exe: target/%/release/csvpp.exe
	mkdir -p $(PREP_DIR)
	cp -R $(DOC_FILES) $(PREP_DIR)
	cp $(TARGET_DIR)/$*/release/csvpp.exe $(PREP_DIR)

# the other platforms don't need a file extension so they can all be packaged the same
$(RELEASE_DIR)/csvpp-$(VERSION)-%/csvpp: $(TARGET_DIR)/%/release/csvpp
	mkdir -p $(PREP_DIR)
	cp -R $(DOC_FILES) $(PREP_DIR)
	cp $(TARGET_DIR)/$*/release/csvpp $(PREP_DIR)
	cp $(TARGET_DIR)/$*/release/csvpp $(PREP_DIR)/csv++

$(RELEASE_DIR)/%.tar.gz: $(RELEASE_DIR)/%/csvpp.exe
$(RELEASE_DIR)/%.tar.gz: $(RELEASE_DIR)/%/csvpp
	cd $(RELEASE_DIR) && tar -czf $*.tar.gz $*

%.asc:
	cd $(RELEASE_DIR) && gpg --detach-sign --armor $*
