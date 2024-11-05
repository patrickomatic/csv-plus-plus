build_dir := book

all: $(build_dir)/index.html $(wildcard $(build_dir)/chapter_*.html)

$(build_dir)/index.html: src/README.md src/SUMMARY.md
	mdbook build

$(build_dir)/%.html: src/%.md
	mdbook build

open:
	open book/index.html

watch:
	mdbook watch

clean:
	rm -rf $(build_dir)/

deps:
	mdbook -V 2>/dev/null || cargo install mdbook

.PHONY: all clean deps open watch
