prepare:
	./configs/git/setup.sh
test-all:
	cargo test
	cargo test -- --ignored
run-examples:
	valgrind -q --leak-check=full --show-leak-kinds=all target/debug/examples/blur-plugin
	valgrind -q --leak-check=full --show-leak-kinds=all target/debug/examples/mirror-plugin