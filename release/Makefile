root_dir := ..
docs_dir := $(root_dir)/docs
target_dir := $(root_dir)/target

version := $(shell git describe --tags --candidates 1)

windows_target := x86_64-pc-windows-gnu
targets := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu \
					 x86_64-apple-darwin aarch64-apple-darwin \
					 $(windows_target)

release_dirs := $(addprefix csvpp-$(version)-, $(targets))
release_files := $(release_dirs:=.tar.gz)
release_file_sigs := $(release_files:=.asc)

doc_files := $(wildcard $(docs_dir)/*.md) ../LICENSE.txt ../README.md
doc_files := $(filter-out $(docs_dir)/RELEASE_CHECKLIST.md, $(doc_files))

.PHONY: all
all: $(release_files) $(release_file_sigs) bump_homebrew_formula_version

.PHONY: bump_homebrew_formula_version
bump_homebrew_formula_version:
	brew bump-formula-pr csv++

.PHONY: clean
clean:
	rm -rf */ *.tar.gz *.asc

$(target_dir)/%/release/csvpp.exe:
$(target_dir)/%/release/csvpp:
	cross build --release --target $*

csvpp-$(version)-%.tar.gz: prep_dir=$(@:.tar.gz=)

# most platforms can be treated the same because they don't have an extension on the final executable
csvpp-$(version)-%.tar.gz: $(target_dir)/%/release/csvpp
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp $< $(prep_dir)/csv++
	cp $< $(prep_dir)/csvpp
	tar -czf $@ $(prep_dir)

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
csvpp-$(version)-$(windows_target).tar.gz: $(target_dir)/$(windows_target)/release/csvpp.exe
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp $< $(prep_dir)
	tar -czf $@ $(prep_dir)

%.tar.gz.asc: %.tar.gz
	gpg --detach-sign --armor $<
