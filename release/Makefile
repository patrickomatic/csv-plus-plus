CARGO_FLAGS := -q

root_dir := ..
docs_dir := $(root_dir)/docs
book_dir := $(root_dir)/book/book
target_dir := $(root_dir)/target

version := $(shell git describe --tags --candidates 1)

# prefer GNU tar otherwise you potentially get weird errors when unpacking it
tar := $(shell command -v gtar || echo "tar")

targets := aarch64-apple-darwin \
					 aarch64-unknown-linux-gnu \
					 aarch64-unknown-linux-musl \
					 i686-unknown-linux-gnu \
					 i686-unknown-linux-musl \
					 i686-pc-windows-gnu \
					 x86_64-apple-darwin \
					 x86_64-pc-windows-gnu \
					 x86_64-unknown-linux-musl \
					 x86_64-unknown-linux-gnu

release_dirs := $(addprefix csvpp-$(version)-, $(targets))
release_files := $(release_dirs:=.tar.gz)
release_file_sigs := $(release_files:=.asc)

doc_files := $(wildcard $(docs_dir)/*.md) ../LICENSE.txt ../README.md
doc_files := $(filter-out $(docs_dir)/RELEASE_CHECKLIST.md, $(doc_files))

all: add_targets \
	install_deps \
	$(release_files) \
	$(release_file_sigs) \
	# bump_homebrew_formula_version

bump_homebrew_formula_version:
	brew bump-formula-pr csv++

clean:
	rm -rf */ *.tar.gz *.asc

add_targets:
	rustup target add $(targets)

install_deps:
	cargo install cross

$(target_dir)/%/release/csvpp $(target_dir)/%/release/csvpp.exe:
	cross build $(CARGO_FLAGS) --release --target $*

csvpp-$(version)-%.tar.gz: prep_dir=$(@:.tar.gz=)

# most platforms can be treated the same because they don't have an extension on the final executable
csvpp-$(version)-%.tar.gz: $(target_dir)/%/release/csvpp
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp -R $(book_dir) $(prep_dir)
	cp $< $(prep_dir)/csv++
	cp $< $(prep_dir)/csvpp
	$(tar) -czf $@ $(prep_dir)

csvpp-$(version)-x86_64-pc-windows-gnu.tar.gz: $(target_dir)/x86_64-pc-windows-gnu/release/csvpp.exe
csvpp-$(version)-i686-pc-windows-gnu.tar.gz: $(target_dir)/i686-pc-windows-gnu/release/csvpp.exe

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
csvpp-$(version)-x86_64-pc-windows-gnu.tar.gz csvpp-$(version)-i686-pc-windows-gnu.tar.gz: csvpp-$(version)-%.tar.gz: $(target_dir)/%/release/csvpp.exe
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp -R $(book_dir) $(prep_dir)
	cp $< $(prep_dir)
	$(tar) -czf $@ $(prep_dir)

%.tar.gz.asc: %.tar.gz
	gpg --detach-sign --armor $<

.PHONY: all add_targets build_book bump_homebrew_formula_version clean install_deps
