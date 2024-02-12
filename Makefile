bench_output := target/criterion

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
	cargo bench --bench eval_fill -- --profile-time=5
	open $(bench_output)/report/index.html
	open $(bench_output)/eval_fill/profile/flamegraph.svg

.PHONY: install
install:
	cargo install --path .

.PHONY: cov
cov:
	cargo llvm-cov
