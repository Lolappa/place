default:
	cargo check
	cargo build
	mkdir target/place -p

clean:
	cargo clean
