TARGET_RELEASE=target/release/pullerman
RUST_SOURCES=$(wildcard src/*.rs)

$(TARGET_RELEASE): $(RUST_SOURCES)
	cargo build --release

install: $(TARGET_RELEASE)
	sudo cp $(TARGET_RELEASE) /usr/local/bin
