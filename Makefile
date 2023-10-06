VERSION=$(shell cat VERSION)

ifeq ($(shell uname -s),Darwin)
CONFIG_DARWIN=y
else ifeq ($(OS),Windows_NT)
CONFIG_WINDOWS=y
else
CONFIG_LINUX=y
endif

LIBRARY_PREFIX=lib
ifdef CONFIG_DARWIN
LOADABLE_EXTENSION=dylib
endif

ifdef CONFIG_LINUX
LOADABLE_EXTENSION=so
endif


ifdef CONFIG_WINDOWS
LOADABLE_EXTENSION=dll
LIBRARY_PREFIX=
endif

prefix=dist
TARGET_LOADABLE=$(prefix)/debug/regex0.$(LOADABLE_EXTENSION)
TARGET_LOADABLE_RELEASE=$(prefix)/release/regex0.$(LOADABLE_EXTENSION)

TARGET_STATIC=$(prefix)/debug/regex0.a
TARGET_STATIC_RELEASE=$(prefix)/release/regex0.a

TARGET_WHEELS=$(prefix)/debug/wheels
TARGET_WHEELS_RELEASE=$(prefix)/release/wheels

INTERMEDIATE_PYPACKAGE_EXTENSION=python/sqlite_regex/sqlite_regex/regex0.$(LOADABLE_EXTENSION)

ifdef target
CARGO_TARGET=--target=$(target)
BUILT_LOCATION=target/$(target)/debug/$(LIBRARY_PREFIX)sqlite_regex.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/$(target)/release/$(LIBRARY_PREFIX)sqlite_regex.$(LOADABLE_EXTENSION)
BUILT_LOCATION_STATIC=target/$(target)/debug/libsqlite_regex.$(STATIC_EXTENSION)
BUILT_LOCATION_STATIC_RELEASE=target/$(target)/release/libsqlite_regex.$(STATIC_EXTENSION)
else
CARGO_TARGET=
BUILT_LOCATION=target/debug/$(LIBRARY_PREFIX)sqlite_regex.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/release/$(LIBRARY_PREFIX)sqlite_regex.$(LOADABLE_EXTENSION)
BUILT_LOCATION_STATIC=target/debug/libsqlite_regex.$(STATIC_EXTENSION)
BUILT_LOCATION_STATIC_RELEASE=target/release/libsqlite_regex.$(STATIC_EXTENSION)
endif

ifdef python
PYTHON=$(python)
else
PYTHON=python3
endif

ifdef IS_MACOS_ARM
RENAME_WHEELS_ARGS=--is-macos-arm
else
RENAME_WHEELS_ARGS=
endif

$(prefix):
	mkdir -p $(prefix)/debug
	mkdir -p $(prefix)/release

$(TARGET_WHEELS): $(prefix)
	mkdir -p $(TARGET_WHEELS)

$(TARGET_WHEELS_RELEASE): $(prefix)
	mkdir -p $(TARGET_WHEELS_RELEASE)

$(TARGET_LOADABLE): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build $(CARGO_TARGET)
	cp $(BUILT_LOCATION) $@

$(TARGET_LOADABLE_RELEASE): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build --release $(CARGO_TARGET)
	cp $(BUILT_LOCATION_RELEASE) $@

$(TARGET_STATIC): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build $(CARGO_TARGET) --features=sqlite-loadable/static
	cp $(BUILT_LOCATION) $@

$(TARGET_STATIC_RELEASE): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build --release $(CARGO_TARGET) --features=sqlite-loadable/static
	cp $(BUILT_LOCATION_RELEASE) $@

python: $(TARGET_WHEELS) $(TARGET_LOADABLE) python/sqlite_regex/setup.py python/sqlite_regex/sqlite_regex/__init__.py .github/workflows/rename-wheels.py
	cp $(TARGET_LOADABLE) $(INTERMEDIATE_PYPACKAGE_EXTENSION)
	rm $(TARGET_WHEELS)/sqlite_regex* || true
	pip3 wheel python/sqlite_regex/ -w $(TARGET_WHEELS)
	python3 .github/workflows/rename-wheels.py $(TARGET_WHEELS) $(RENAME_WHEELS_ARGS)

python-release: $(TARGET_LOADABLE_RELEASE) $(TARGET_WHEELS_RELEASE) python/sqlite_regex/setup.py python/sqlite_regex/sqlite_regex/__init__.py .github/workflows/rename-wheels.py
	cp $(TARGET_LOADABLE_RELEASE)  $(INTERMEDIATE_PYPACKAGE_EXTENSION)
	rm $(TARGET_WHEELS_RELEASE)/sqlite_regex* || true
	pip3 wheel python/sqlite_regex/ -w $(TARGET_WHEELS_RELEASE)
	python3 .github/workflows/rename-wheels.py $(TARGET_WHEELS_RELEASE) $(RENAME_WHEELS_ARGS)

datasette: $(TARGET_WHEELS) python/datasette_sqlite_regex/setup.py python/datasette_sqlite_regex/datasette_sqlite_regex/__init__.py
	rm $(TARGET_WHEELS)/datasette* || true
	pip3 wheel python/datasette_sqlite_regex/ --no-deps -w $(TARGET_WHEELS)

datasette-release: $(TARGET_WHEELS_RELEASE) python/datasette_sqlite_regex/setup.py python/datasette_sqlite_regex/datasette_sqlite_regex/__init__.py
	rm $(TARGET_WHEELS_RELEASE)/datasette* || true
	pip3 wheel python/datasette_sqlite_regex/ --no-deps -w $(TARGET_WHEELS_RELEASE)

bindings/sqlite-utils/pyproject.toml: bindings/sqlite-utils/pyproject.toml.tmpl VERSION
	VERSION=$(VERSION) envsubst < $< > $@
	echo "✅ generated $@"

bindings/sqlite-utils/sqlite_utils_sqlite_regex/version.py: bindings/sqlite-utils/sqlite_utils_sqlite_regex/version.py.tmpl VERSION
	VERSION=$(VERSION) envsubst < $< > $@
	echo "✅ generated $@"

sqlite-utils: $(TARGET_WHEELS) bindings/sqlite-utils/pyproject.toml bindings/sqlite-utils/sqlite_utils_sqlite_regex/version.py
	python3 -m build bindings/sqlite-utils -w -o $(TARGET_WHEELS)

sqlite-utils-release: $(TARGET_WHEELS) bindings/sqlite-utils/pyproject.toml bindings/sqlite-utils/sqlite_utils_sqlite_regex/version.py
	python3 -m build bindings/sqlite-utils -w -o $(TARGET_WHEELS_RELEASE)

npm: VERSION npm/platform-package.README.md.tmpl npm/platform-package.package.json.tmpl npm/sqlite-regex/package.json.tmpl scripts/npm_generate_platform_packages.sh
	scripts/npm_generate_platform_packages.sh

deno: VERSION deno/deno.json.tmpl
	scripts/deno_generate_package.sh

Cargo.toml: VERSION
	cargo set-version `cat VERSION`

python/sqlite_regex/sqlite_regex/version.py: VERSION
	printf '__version__ = "%s"\n__version_info__ = tuple(__version__.split("."))\n' `cat VERSION` > $@

python/datasette_sqlite_regex/datasette_sqlite_regex/version.py: VERSION
	printf '__version__ = "%s"\n__version_info__ = tuple(__version__.split("."))\n' `cat VERSION` > $@

bindings/ruby/lib/version.rb: bindings/ruby/lib/version.rb.tmpl VERSION
	VERSION=$(VERSION) envsubst < $< > $@

ruby: bindings/ruby/lib/version.rb

version:
	make Cargo.toml
	make python/sqlite_regex/sqlite_regex/version.py
	make python/datasette_sqlite_regex/datasette_sqlite_regex/version.py
	make bindings/sqlite-utils/pyproject.toml bindings/sqlite-utils/sqlite_utils_sqlite_regex/version.py
	make npm
	make deno
	make ruby


format:
	cargo fmt

sqlite-regex.h: cbindgen.toml
	rustup run nightly cbindgen  --config $< -o $@

release: $(TARGET_LOADABLE_RELEASE) $(TARGET_STATIC_RELEASE)

loadable: $(TARGET_LOADABLE)
loadable-release: $(TARGET_LOADABLE_RELEASE)

static: $(TARGET_STATIC)
static-release: $(TARGET_STATIC_RELEASE)

debug: loadable static python datasette
release: loadable-release static-release python-release datasette-release

clean:
	rm dist/*
	cargo clean

test-loadable:
	$(PYTHON) tests/test-loadable.py

test-python:
	$(PYTHON) tests/test-python.py

test-npm:
	node npm/sqlite-regex/test.js

test-deno:
	deno task --config deno/deno.json test

test:
	make test-loadable
	make test-python
	make test-npm
	make test-deno

publish-release:
	./scripts/publish_release.sh

.PHONY: clean \
	test test-loadable test-python test-npm test-deno \
	loadable loadable-release \
	python python-release \
	datasette datasette-release \
	sqlite-utils sqlite-utils-release \
	static static-release \
	debug release \
	format version publish-release \
	npm deno ruby
