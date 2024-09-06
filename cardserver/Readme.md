# first time build
`$ sudo apt-get install musl-tools`
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
$ cargo test && cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/bootstrap bootstrap && zip lambda.zip bootstrap && aws s3 cp ./lambda.zip s3://card-server-code-bucket-619648504467/lambda.zip && aws lambda update-function-code --function-name ServiceStack-CardLambda1A48BD24-ICVfF7fvVd2y --s3-bucket card-server-code-bucket-619648504467 --s3-key lambda.zip
```

# test aws lambda
```
$ aws lambda invoke --function-name ServiceStack-CardLambda1A48BD24-ICVfF7fvVd2y --payload '{"name": "Black Lotus"}' output.json && cat output.json
```

# to do
figure out how to alias that command