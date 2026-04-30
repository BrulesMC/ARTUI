PREFIX=/usr
BINDIR=$(PREFIX)/bin

# Get real user even when running with sudo
REAL_USER := $(shell echo $${SUDO_USER:-$$USER})

# Resolve home directory safely
USER_HOME := $(shell getent passwd $(REAL_USER) | cut -d: -f6)

CONFIGDIR := $(USER_HOME)/.config
CONFIGFILE := $(CONFIGDIR)/ar.json

SUDOERS_FILE=/etc/sudoers.d/artools-ryzenadj

all:
	cargo build --release --bin artui
	cargo build --release --bin arctl

install: all
	# install binaries (system-wide)
	sudo install -Dm755 target/release/artui $(BINDIR)/artui
	sudo install -Dm755 target/release/arctl $(BINDIR)/arctl

	# ensure config dir exists as real user
	sudo -u $(REAL_USER) mkdir -p $(CONFIGDIR)

	# copy config only if missing (as real user)
	sudo -u $(REAL_USER) sh -c '[ -f "$(CONFIGFILE)" ] || cp config.json "$(CONFIGFILE)"'

	# sudo rule for ryzenadj
	printf "%s ALL=(ALL) NOPASSWD: /usr/bin/ryzenadj\n" "$(REAL_USER)" \
		| sudo tee $(SUDOERS_FILE) >/dev/null
	sudo chmod 440 $(SUDOERS_FILE)

	@echo "installed"

uninstall:
	sudo rm -f $(BINDIR)/artui
	sudo rm -f $(BINDIR)/arctl
	rm -f $(CONFIGFILE)
	sudo rm -f $(SUDOERS_FILE)
	@echo "uninstalled"

clean:
	cargo clean
