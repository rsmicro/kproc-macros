CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all

example:
	@echo "nothing yet!"

clean:
	$(CC) clean

book:
	cd docs/docs-book; mdbook build

dev-book:
	cd docs/docs-book; mdbook serve --open
