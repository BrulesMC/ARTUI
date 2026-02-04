PREFIX=/usr
BINDIR=$(PREFIX)/bin
CONFIGDIR=$(HOME)/.config
CONFIGFILE=$(CONFIGDIR)/ar.json
SUDOERS_FILE=/etc/sudoers.d/artools-ryzenadj
all:
	cargo build --release --bin artui
	cargo build --release --bin arctl
install: all
	sudo install -Dm755 target/release/artui $(BINDIR)/artui
	sudo install -Dm755 target/release/arctl $(BINDIR)/arctl
	mkdir -p $(CONFIGDIR)
	[ -f $(CONFIGFILE) ] || cp -f config.json $(CONFIGFILE)
	printf "%s ALL=(ALL) NOPASSWD: /usr/bin/ryzenadj\n" "$(USER)" \
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
