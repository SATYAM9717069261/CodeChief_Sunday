main: main.rs libcommand.so
	rustc -L . main.rs

libcommand.so: commandMacro.rs
	rustc --crate-type proc-macro commandMacro.rs

