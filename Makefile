#
# Maintenance Helpers
#
# This makefile contains targets used for development, as well as helpers to
# aid automatization of maintenance.
#

BUILDDIR ?= .
SRCDIR ?= .

BIN_CARGO ?= cargo
BIN_CLANG ?= clang-11
BIN_LLD ?= lld
BIN_MKDIR ?= mkdir
BIN_OBJDUMP ?= objdump

SHELL = /bin/bash

BUILDDIR_ABS := $(abspath $(BUILDDIR))
SRCDIR_ABS := $(abspath $(SRCDIR))

#
# Generic Targets
#
# The following is a set of generic targets used across the makefile. The
# following targets are defined:
#
#     help
#         This target prints all supported targets. This is also the default
#         target.
#
#     $(BUILDDIR)/
#     $(BUILDDIR)/%/
#         This target simply creates the specified directory. It is limited to
#         the build-dir as a safety measure. Note that this requires you to use
#         a trailing slash after the directory to not mix it up with regular
#         files. Lastly, you mostly want this as order-only dependency, since
#         timestamps on directories do not affect their content.
#

.PHONY: help
help:
	@echo "make [TARGETS...]"
	@echo
	@echo "This is a project-maintenance makefile. The following"
	@echo "targets are available:"
	@echo
	@echo "    help:               Print this usage information."

$(BUILDDIR)/:
	$(BIN_MKDIR) -p "$@"

$(BUILDDIR)/%/:
	$(BIN_MKDIR) -p "$@"

#
# Test: syscall xLTO
#
# This provides 2 test targets for the `freestanding-syscall` example. It
# compiles the target as freestanding binary with and without `xLTO`
# (cross-language link-time-optimization). It then verifies that the
# disassembly correctly reflects the inlined or non-inlined syscall.
#

TEST_SYSCALL_FILTER_SYMBOLS = \
	grep \
		-E \
		"^[^ ]+ <[^ ]+>:" \
	| sed \
		-e \
		's/^[^ ]\+ //g'

.PHONY: test-syscall-xlto
test-syscall-xlto: | $(BUILDDIR)/
	( \
		CC=$(BIN_CLANG) \
		CFLAGS=-flto=thin \
			$(BIN_CARGO) +nightly rustc \
				--example="freestanding-syscall" \
				--features="freestanding,unstable" \
				--release \
				--target-dir="$(BUILDDIR_ABS)/target-syscall-xlto" \
				-vv \
				-- \
					-C "linker=$(BIN_CLANG)" \
					-C "link-arg=-fuse-ld=$(BIN_LLD)" \
					-C "link-arg=-nostartfiles" \
					-C "linker-plugin-lto=yes" \
	)
	( \
		SYMS_REAL="$$( \
			$(BIN_OBJDUMP) \
				-d \
				"$(BUILDDIR)/target-syscall-xlto/release/examples/freestanding-syscall" \
			| $(TEST_SYSCALL_FILTER_SYMBOLS) \
		)" ; \
		SYMS_EXPECTED="<_start>:" ; \
		echo -e "Symbols:\n$${SYMS_REAL}" ; \
		echo -e "Symbols (expected):\n$${SYMS_EXPECTED}" ; \
		test "$${SYMS_REAL}" = "$${SYMS_EXPECTED}" \
	)

.PHONY: test-syscall-no-xlto
test-syscall-no-xlto: | $(BUILDDIR)/
	( \
		$(BIN_CARGO) +nightly rustc \
			--example="freestanding-syscall" \
			--features="freestanding,unstable" \
			--target-dir="$(BUILDDIR_ABS)/target-syscall-no-xlto" \
			-vv \
			-- \
				-C link-arg=-nostartfiles \
	)
	( \
		SYMS_REAL="$$( \
			$(BIN_OBJDUMP) \
				-d \
				"$(BUILDDIR)/target-syscall-no-xlto/debug/examples/freestanding-syscall" \
			| $(TEST_SYSCALL_FILTER_SYMBOLS) | grep "r_linux" \
		)" ; \
		SYMS_EXPECTED="<r_linux_asm_syscall1>:" ; \
		echo -e "Symbols:\n$${SYMS_REAL}" ; \
		echo -e "Symbols (expected):\n$${SYMS_EXPECTED}" ; \
		test "$${SYMS_REAL}" = "$${SYMS_EXPECTED}" \
	)
