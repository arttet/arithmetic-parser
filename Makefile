.PHONY: help
help:	## Show this help
	@fgrep -h "## " $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

.PHONY: coverage
coverage:	## Generate code coverage report
	RUSTFLAGS="-Cinstrument-coverage" CARGO_INCREMENTAL=0 LLVM_PROFILE_FILE=target/coverage-%p-%m.profraw cargo test --verbose --tests
	grcov . --binary-path ./target/debug/ --source-dir . --log-level ERROR --branch --ignore-not-existing --ignore "*cargo*" -t html -o ./target/coverage/
	grcov . --binary-path ./target/debug/ --source-dir . --log-level ERROR --branch --ignore-not-existing --ignore "*cargo*" -t lcov -o ./target/coverage.lcov
