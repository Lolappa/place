default:
	cargo build
	
	mkdir -p target/place
	mkdir -p target/place/place
	
	mkdir -p target/place/backend
	cp target/debug/place_backend target/place/backend
	
	mkdir -p target/place/data
	mkdir -p target/place/data/dir
	mkdir -p target/place/data/file
	
	cp target/debug/place_cat target/place/cat
	cp target/debug/place_print target/place/print
	cp target/debug/place_mkfile target/place/mkfile
	cp target/debug/place_setbyte target/place/setbyte

clean:
	cargo clean
