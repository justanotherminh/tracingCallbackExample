all:
	cargo build --release
	gcc src/main.c target/release/libtracingCallbackExample.a -o main -ldl -lpthread -lm

clean:
	rm main
	rm -rf target
