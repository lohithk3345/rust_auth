all: build user_auth

build:
	@cargo build
	@cp target/debug/user_auth bin/user_auth_d

clean:
	@cargo clean

user_auth:
	@cargo build --release
	@cp target/release/user_auth bin/user_auth
