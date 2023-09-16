RELEASE_DIR := release
TARGET_DIR := target

VERSION := $(shell git describe --tags --candidates 1)

TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu \
					 x86_64-apple-darwin aarch64-apple-darwin \
					 x86_64-pc-windows-gnu

# the .tar.gz files and relevant signatures
RELEASE_DIRS := $(addprefix $(RELEASE_DIR)/csvpp-$(VERSION)-, $(TARGETS))
RELEASE_FILES := $(RELEASE_DIRS:=.tar.gz)
RELEASE_FILE_SIGS := $(RELEASE_FILES:=.asc)

# the .deb packages that we'll generate (and their signature files)
DEB_TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
DEB_RELEASE_DIRS := $(addprefix $(RELEASE_DIR)/csvpp-$(VERSION)-, $(DEB_TARGETS))
DEB_RELEASE_FILES := $(DEB_RELEASE_DIRS:=.deb)
DEB_RELEASE_FILE_SIGS := $(DEB_RELEASE_FILES:=.asc)

.PHONY: all
# TODO: cargo-deb doesn't work yet
# all: tool-deps $(RELEASE_FILES) $(RELEASE_FILE_SIGS) $(DEB_RELEASE_FILES)
all: tool-deps $(RELEASE_FILES) $(RELEASE_FILE_SIGS)

.PHONY: tool-deps
tool-deps:
	cargo install cross cargo-deb

$(TARGET_DIR)/%/release/csvpp.exe:
$(TARGET_DIR)/%/release/csvpp:
	cross build --release --target $*

$(RELEASE_DIR)/csvpp-$(VERSION)-%.tar.gz: RELEASE_DIR=$(@:.tar.gz=)

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
$(RELEASE_DIR)/csvpp-$(VERSION)-x86_64-pc-windows-gnu/csvpp.exe: target/x86_64-pc-windows-gnu/release/csvpp.exe
	mkdir -p $(RELEASE_DIR)
	cp -R docs/ LICENSE.txt README.md $(RELEASE_DIR)
	cp $(TARGET_DIR)/x86_64-pc-windows-gnu/release/csvpp.exe $(RELEASE_DIR)

# the other platforms don't need a file extension so they can all be packaged the same
$(RELEASE_DIR)/csvpp-$(VERSION)-%/csvpp: $(TARGET_DIR)/%/release/csvpp
	mkdir -p $(RELEASE_DIR)
	cp -R docs/ LICENSE.txt README.md $(RELEASE_DIR)
	cp $(TARGET_DIR)/$*/release/csvpp $(RELEASE_DIR)
	cp $(TARGET_DIR)/$*/release/csvpp $(RELEASE_DIR)/csv++

$(RELEASE_DIR)/%.tar.gz: $(RELEASE_DIR)/%/csvpp.exe
$(RELEASE_DIR)/%.tar.gz: $(RELEASE_DIR)/%/csvpp
	cd $(RELEASE_DIR) && tar -czf $*.tar.gz $*

$(RELEASE_DIR)/csvpp-$(VERSION)-%.deb: $(TARGET_DIR)/%/release/csvpp
# --no-build because we already built it with cross. and --no-strip because Cargo.toml is already
#  configured to strip debug symbols
	cross deb --no-build --no-strip --target $*

%.asc:
	cd $(RELEASE_DIR) && gpg --detach-sign --armor $*

.PHONY: clean
clean:
	cargo clean --release
	rm -rf $(RELEASE_FILES) $(RELEASE_FILE_SIGS) $(RELEASE_DIRS)
