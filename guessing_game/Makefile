# add crates to the project
add: 
	cargo add $(package)
#  build your project
build: 
		cargo build

# cargo check is much faster than cargo build, because it skips the step of producing an executable.
check: 
		cargo check

# to compile the code and then run the resulting executable all in one command
run:
		cargo run

# the executable with this command
run_2:	target
		./target/debug/hello_cargo

# Building for release of the package
build-release: cargo
	cargo build --release
