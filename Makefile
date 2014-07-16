build:
	mkdir -p build
	rustc src/main.rs -o build/tinier
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/tinier
.PHONY: build
