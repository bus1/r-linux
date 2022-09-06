#
# Maintenance Helpers
#
# This makefile contains targets used for development, as well as helpers to
# aid automatization of maintenance.
#

BUILDDIR ?= .
SRCDIR ?= .

BIN_MKDIR ?= mkdir

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
