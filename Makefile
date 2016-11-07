CARGO_FLAGS?=
TASK=target/debug/teleinfo
SRC=Cargo.toml src/main.rs src/teleinfo.rs

ifeq ($(APP_ENVIRONMENT),prod)
	TASK=target/release/teleinfo
	CARGO_FLAGS+=--release
endif

all: $(TASK)

$(TASK): $(SRC)
	cargo build $(CARGO_FLAGS)

.PHONY: all
