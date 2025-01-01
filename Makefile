include *.config

default:
	INTERVAL=${interval} SIZE_X=${size_x} SIZE_Y=${size_y} cargo build
	
	mkdir -p target/place
	mkdir -p target/place/place
	
	mkdir -p target/place/backend
	touch target/place/backend/user_timestamps
	cp target/debug/place_backend target/place/backend
	
	mkdir -p target/place/data
	mkdir -p target/place/data/folders
	mkdir -p target/place/data/files
	touch target/place/data/data
	
	cp target/debug/place_cat target/place

clean:
	cargo clean
