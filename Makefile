all: build-release

BIN_DIR = ~/bin
RELEASE_DIR = release

VERSION := $(shell git describe --tags --candidates 1)

define tar_release
	cp target/$(1)/release/csvpp$(2) $(RELEASE_DIR)/csvpp
	cp target/$(1)/release/csvpp$(2) $(RELEASE_DIR)/csv++
	cp LICENSE.txt README.md $(RELEASE_DIR)/

	# include README, docs and license?
	cd $(RELEASE_DIR) && tar -czf csvpp-$(VERSION)-$(1).tar.gz csvpp csv++ \
		&& rm csvpp csv++
endef

install:
	[ -d $(BIN_DIR) ] || mkdir -p $(BIN_DIR)

	cargo build --profile release
	cp target/release/csvpp ~/bin/csvpp
	cp target/release/csvpp ~/bin/csv++

build-release:
	[ -d $(RELEASE_DIR) ] || mkdir -p $(RELEASE_DIR)

	# Linux
	cross build --release --target x86_64-unknown-linux-gnu
	$(call tar_release,x86_64-unknown-linux-gnu,"")
	cross build --release --target aarch64-unknown-linux-gnu
	$(call tar_release,aarch64-unknown-linux-gnu,"")

	# OS X
	cross build --release --target x86_64-apple-darwin
	$(call tar_release,x86_64-apple-darwin,"")
	cross build --release --target aarch64-apple-darwin
	$(call tar_release,aarch64-apple-darwin,"")

	# Windows
	cross build --release --target x86_64-pc-windows-gnu
	$(call tar_release,x86_64-pc-windows-gnu,".exe")

clean:
	rm -rf target/x86_64-pc-windows-gnu \
		target/x86_64-unknown-linux-gnu \
		target/aarch64-unknown-linux-gnu \
		target/x86_64-apple-darwin \
		target/aarch64-apple-darwin \
		$(RELEASE_DIR)/README.md \
		$(RELEASE_DIR)/LICENSE.txt
