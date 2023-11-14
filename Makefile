bench_report := target/criterion/report/index.html

.PHONY: all
all:
	cargo test
	cargo bench && open $(bench_report)

.PHONY: bench
bench:
	cargo bench && open $(bench_report)

.PHONY: cov
cov:
	cargo llvm-cov
