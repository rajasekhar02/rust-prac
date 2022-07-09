#  build your project
build: cargo
		cargo build

# cargo check is much faster than cargo build, because it skips the step of producing an executable.
check: cargo
		cargo check

# to compile the code and then run the resulting executable all in one command
run:	cargo 
		cargo run
# the executable with this command
run_2:	target
		./target/debug/hello_cargo