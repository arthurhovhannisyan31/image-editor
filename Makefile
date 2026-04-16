prepare:
	./configs/git/setup.sh
test-all:
	cargo test
	cargo test -- --ignored
check-miri:
	cargo miri test
check-valgrind:
	valgrind --leak-check=full --show-leak-kinds=all cargo run --example mirror-plugin
	valgrind --leak-check=full --show-leak-kinds=all cargo run --example blur-plugin