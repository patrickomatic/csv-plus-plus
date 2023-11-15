bench_report := target/criterion/report/index.html

# TODO: why does this build each one individually?
.PHONY: all
all:
	cargo doc --lib
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	cargo test
	make -C release/

.PHONY: bench
bench:
	cargo bench && open $(bench_report)

.PHONY: cov
cov:
	cargo llvm-cov
