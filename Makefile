release_dir := release
target_dir := target

version := $(shell git describe --tags --candidates 1)

windows_target := x86_64-pc-windows-gnu
targets := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu \
					 x86_64-apple-darwin aarch64-apple-darwin \
					 $(windows_target)

# the .tar.gz files and relevant signatures
release_dirs := $(addprefix $(release_dir)/csvpp-$(version)-, $(targets))
release_files := $(release_dirs:=.tar.gz)
release_file_sigs := $(release_files:=.asc)

windows_release_dir := csvpp-$(version)-$(windows_target)

doc_files := $(wildcard docs/*.md) LICENSE.txt README.md
doc_files := $(filter-out docs/RELEASE_CHECKLIST.md, $(doc_files))

.PHONY: all
all: $(release_files) $(release_file_sigs)

.PHONY: clean
clean:
	cargo clean --release
	rm -rf $(release_files) $(release_file_sigs) $(release_dirs)

$(target_dir)/%/$(release_dir)/csvpp.exe:
$(target_dir)/%/$(release_dir)/csvpp:
	cross build --release --target $*

$(release_dir)/csvpp-$(version)-%.tar.gz: prep_dir=$(@:.tar.gz=)

# most platforms can be treated the same because they don't have an extension on the final executable
$(release_dir)/csvpp-$(version)-%/csvpp: $(target_dir)/%/release/csvpp
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp $(target_dir)/$*/release/csvpp $(prep_dir)/csv++
	cp $(target_dir)/$*/release/csvpp $(prep_dir)

$(release_dir)/%.tar.gz: $(release_dir)/%/csvpp
	cd $(release_dir) && tar -czf $*.tar.gz $*

# we need special handling for windows because we're producing something with an .exe extension. we
# also don't package csv++.exe (maybe we should? just seems kinda odd)
$(release_dir)/$(windows_release_dir)/csvpp.exe: target/$(windows_target)/release/csvpp.exe
	mkdir -p $(prep_dir)
	cp -R $(doc_files) $(prep_dir)
	cp $(target_dir)/$(windows_target)/release/csvpp.exe $(prep_dir)

$(release_dir)/$(windows_release_dir).tar.gz: $(release_dir)/$(windows_release_dir)/csvpp.exe
	cd $(release_dir) && tar -czf $(windows_release_dir).tar.gz $(windows_release_dir)

%.tar.gz.asc: %.tar.gz
	gpg --detach-sign --armor $<
