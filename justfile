export RUSTDOCFLAGS := "-D warnings"

default: check

fmt:
    cargo fmt --all

check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo doc --no-deps --all-features
