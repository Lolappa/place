include *.config

default:
	mkdir target/place/backend -p
	INTERVAL=${interval} cargo build
	cp target/debug/place_backend target/place/backend
	cp target/debug/place_cat target/place
clean:
	cargo clean
