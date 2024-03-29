RUST_BACKTRACE ?= 0

setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs test-and-run-day%

run-all:
	@cargo run --release --bin aoc --quiet

test-libs:
	@cargo nextest run --lib --cargo-quiet

test-and-run-day%: day = $(subst test-and-run-,,$@)
test-and-run-day%:
	@echo "$(setaf6)>>>>> Testing ${day} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${day} --no-fail-fast --cargo-quiet

	@echo "$(setaf6)>>>>> Running ${day} <<<<<$(sgr0)"
	@cargo run --bin ${day} --release --quiet
