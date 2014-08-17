build:
	mkdir -p build
	rustc src/main.rs -o build/tinier -L rustVM/build/
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/tinier
.PHONY: build
