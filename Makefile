include *.config

default:
	echo "pub const INTERVAL: usize = ${interval};" > constants/constants.rs
	echo "pub const SIZE_X: usize = ${size_x};" >> constants/constants.rs
	echo "pub const SIZE_Y: usize = ${size_y};" >> constants/constants.rs

	cargo build
	
	mkdir -p target/place
	mkdir -p target/place/place
	
	mkdir -p target/place/backend
	touch target/place/backend/user_timestamps
	cp target/debug/place_backend target/place/backend
	
	mkdir -p target/place/data
	mkdir -p target/place/data/folders
	mkdir -p target/place/data/files
	
	cp target/debug/place_cat target/place

clean:
	cargo clean
