# Run unit and doc tests
[group('test')]
test test_name="":
  cargo test --lib --features github-ci {{ test_name }}
  cargo test --doc

# Generate an LCOV coverage report.
[group('test')]
coverage output="lcov.info" args="":
  cargo llvm-cov \
    --no-fail-fast \
    --features github-ci \
    --lib \
    --lcov \
    --output-path {{ output }} \
    {{ args }}
