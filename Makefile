waymenu:
	cargo build --release

node_modules:
	npm i

waymenu.%: waymenu.%.md node_modules
	npx marked-man $< --no-breaks > target/$@

install: waymenu waymenu.1 waymenu.5
	install -m 755 target/release/waymenu $(DESTDIR)/bin/waymenu
	mkdir -p $(DESTDIR)/share/man/man1
	install -m 644 target/waymenu.1 $(DESTDIR)/share/man/man1/waymenu.1
	mkdir -p $(DESTDIR)/share/man/man5
	install -m 644 target/waymenu.5 $(DESTDIR)/share/man/man5/waymenu.5