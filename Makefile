PROJECTROOT := $(shell pwd)
KERNELDIR := $(PROJECTROOT)/kernel

# Make is verbose in Linux. Make it silent.
MAKEFLAGS += --silent

.PHONY: default
default: help
.ONESHELL: # Only applies to all target

install:
	@rustup override set nightly
	@rustup component add rust-src
	@rustup component add llvm-tools-preview
	
fmt:
	@printf "🔧 Formatting\n"
	cd $(KERNELDIR)
	@printf "👍 Done\n"

kernel_build:
	@printf "🔧 Building kernel binary\n"
	cd $(KERNELDIR)
	@cargo install bootimage
	@cargo build
	@printf "👍 Done\n"

kernel_test:
	@printf "🔧 Running kernel tests\n"
	cd $(KERNELDIR)
	@cargo test
	@printf "👍 Done\n"

kernel_run:
	@printf "🔧 Updating crates\n"
	cd $(KERNELDIR)
	@printf "⛓️ Attaching runner\n"
	@printf "🔨 Running QEMU\n"
	@cargo run
	@printf "👍 Done\n"