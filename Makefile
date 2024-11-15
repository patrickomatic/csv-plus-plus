bench_output := target/criterion

.PHONY: all bench buildci buildrelease install cov

all: buildrelease

buildci:
	cargo clippy --workspace -- -D warnings
	cargo fmt --all -- --check
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

buildrelease:
	cargo doc --lib
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	cargo test
	make -C book/
	make -C release/
	# make -j 5 -C release/

bench:
	RUST_BACKTRACE=1 cargo bench --bench eval_fill -- --profile-time=5
	open $(bench_output)/report/index.html
	open $(bench_output)/eval_fill/profile/flamegraph.svg

install:
	cargo install --path .

cov:
	cargo llvm-cov
