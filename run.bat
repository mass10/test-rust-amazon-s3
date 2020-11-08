@SETLOCAL

@SET AWS_ACCESS_KEY_ID=
@SET AWS_SECRET_ACCESS_KEY=

@CALL cargo fmt
@CALL cargo run --release --quiet
