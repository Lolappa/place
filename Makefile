include *.config

default:
	echo "pub const INTERVAL: usize = ${interval};" > constants/constants.rs
	echo "pub const TOKEN: &str = \"${token}\";" >> constants/constants.rs
	echo "pub const SIZE_X: usize = ${size_x};" >> constants/constants.rs
	echo "pub const SIZE_Y: usize = ${size_y};" >> constants/constants.rs
	echo "pub const LOCATION: &str = \"${location}\";" >> constants/constants.rs
	echo "pub const SOCK_LOCATION: &str = \"${location}.sock\";" >> constants/constants.rs

	cargo build
	
	mkdir -p target/place
	mkdir -p target/place/place
	
	mkdir -p target/place/backend
	touch target/place/backend/user_timestamps
	cp target/debug/place_backend target/place/backend
	
	mkdir -p target/place/data
	mkdir -p target/place/data/folders
	mkdir -p target/place/data/files
	
	cp target/debug/place_cat target/place/cat
	cp target/debug/place_print target/place/print
	cp target/debug/place_mkfile target/place/mkfile

clean:
	cargo clean
