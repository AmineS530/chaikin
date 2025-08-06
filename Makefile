BINARY_NAME := chaikin

all: run

build:
	@echo "\033[0;36m==> Building in release mode...\033[0m"
	@cargo build --release
	@echo "\033[0;32m==> Copying binary to current directory: $(BINARY_NAME)\033[0m"
	@cp target/release/$(BINARY_NAME) ./$(BINARY_NAME)

run: build
	@echo "\033[1;33m==> Running $(BINARY_NAME)...\033[0m"
	@./$(BINARY_NAME)

clean:
	@echo "\033[0;36m==> Cleaning build artifacts...\033[0m"
	@cargo clean
	@rm -f ./$(BINARY_NAME)

.PHONY: build run clean
