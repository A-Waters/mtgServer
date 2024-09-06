# testing
```
$ cargo test
```

for print logs
```
cargo test -- --nocapture
```

# runninng local
```
$ cargo run
```

# first time build

`$ rustup target add x86_64-unknown-linux-musl`

# every build after
```
$ cargo test
$ cargo build --release --target x86_64-unknown-linux-musl
$ cp target/x86_64-unknown-linux-musl/release/bootstrap bootstrap
$ zip lambda.zip bootstrap
```
or
```
$ cargo test && cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/bootstrap bootstrap && zip lambda.zip bootstrap
```

# then upload lambda.zip to aws lambda console