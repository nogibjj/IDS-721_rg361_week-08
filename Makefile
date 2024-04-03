test:
	find . -type f -name "*.toml" -exec cargo test  --quiet --manifest-path {} \;

format:	
	find . -type f -name "*.toml" -exec cargo fmt  --quiet --manifest-path {} \;

lint:
	find . -type f -name "*.toml" -exec cargo clippy  --quiet --manifest-path {} \;
build:
	cargo build --release --manifest-path ./rust_cli/Cargo.toml

copy:
	cp rust_cli/target/release/rust_cli ./"Rust_Binary"
		
all: build test format lint
