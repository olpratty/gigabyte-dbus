.DEFAULT_GOAL := build
DESTDIR ?=
DEBUG ?= 0

ifeq ($(DEBUG),0)
ARGS := --release
TARGET := release
else
ARGS :=
TARGET := debug
endif

.PHONY: all clean build install uninstall

all: build

clean:
	cargo clean

build:
	cargo build --locked $(ARGS) --bin gigabyted

# Build first. Account creation and service activation are separate operations.
install:
	test -f "target/$(TARGET)/gigabyted"
	install -Dm755 "target/$(TARGET)/gigabyted" "$(DESTDIR)/usr/bin/gigabyted"
	install -Dm644 gigabyted.service "$(DESTDIR)/usr/lib/systemd/system/gigabyted.service"
	install -Dm644 gigabyted.conf "$(DESTDIR)/usr/share/dbus-1/system.d/gigabyted.conf"
	install -Dm644 gigabyted.sysusers "$(DESTDIR)/usr/lib/sysusers.d/gigabyted.conf"
	install -Dm644 LICENSE "$(DESTDIR)/usr/share/licenses/gigabyte-dbus/LICENSE"

uninstall:
	rm -f "$(DESTDIR)/usr/bin/gigabyted"
	rm -f "$(DESTDIR)/usr/lib/systemd/system/gigabyted.service"
	rm -f "$(DESTDIR)/usr/share/dbus-1/system.d/gigabyted.conf"
	rm -f "$(DESTDIR)/usr/lib/sysusers.d/gigabyted.conf"
	rm -f "$(DESTDIR)/usr/share/licenses/gigabyte-dbus/LICENSE"
