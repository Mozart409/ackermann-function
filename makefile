NAME=ackermann-function

clear:
	clear

build: clear
	cargo build --release

run: 
	./target/release/$(NAME)

start: build run