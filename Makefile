PROJECTROOT := $(shell pwd)
KERNELDIR := $(PROJECTROOT)/kernel

# Make is verbose in Linux. Make it silent.
MAKEFLAGS += --silent

.PHONY: default
default: help
.ONESHELL: # Only applies to all target

## install: Install toolchain dependencies
install:
	@rustup override set nightly-2021-12-07
	@rustup component add rust-src
	@rustup component add llvm-tools-preview
	@rustup component add clippy

## fmt: Format codebase using cargo fmt
fmt:
	@printf "🔧 Formatting\n"
	cd $(KERNELDIR)
	@cargo fmt --all
	@printf "👍 Done\n"

## kernel_build: Build kernel image
kernel_build:
	@printf "🔧 Building kernel binary\n"
	cd $(KERNELDIR)
	@cargo install bootimage
	@cargo build
	@printf "👍 Done\n"

## kernel_test: Run kernel tests
kernel_test:
	@printf "🔧 Running kernel tests\n"
	cd $(KERNELDIR)
	@cargo test
	@printf "👍 Done\n"

## kernel_run: Attach QEMU and run kernel
kernel_run:
	@printf "🔧 Updating crates\n"
	cd $(KERNELDIR)
	@printf "⛓️ Attaching runner\n"
	@printf "🔨 Running QEMU\n"
	@cargo run
	@printf "👍 Done\n"

help: Makefile
	@printf "\nRusticOS, Lightweight OS implementation in Rust\n\n"
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@printf "\nDo check out the code at https://github.com/sdslabs/rusticos\n\n"
