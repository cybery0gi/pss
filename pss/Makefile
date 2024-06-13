# Variables
PROJECT_NAME=pss

BUILD_DIR=target
CARGO=cargo
SRC_DIR=src

# Targets
.PHONY: all clean build run

# Default target
all: build

# Build the project
build:
	@echo "Building the project..."
	@$(CARGO) build

# Run the project
run: build
	@echo "Running the project..."
	@$(CARGO) run

# Clean the project
clean:
	@echo "Cleaning the project..."
	@$(CARGO) clean

# Help message
help:
	@echo "Makefile targets:"
	@echo "  all    - Build the project (default target)"
	@echo "  build  - Build the project"
	@echo "  run    - Build and run the project"
	@echo "  clean  - Clean the project"
	@echo "  help   - Display this help message"
