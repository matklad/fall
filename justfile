test:
    cargo test --all

performance:
    slow_tests=1 cargo test --release --package lang_rust --test test -- performance_test

bootstrap:
    rewrite=bootstrap cargo test --package fall_gen --test cli
    cargo test --all

generate-parsers:
    rewrite=parsers cargo test --package fall_gen --test cli
    cargo test -p fall_test -p fall_test -p lang_rust -p lang_json

update-test-data:
    rewrite_test_data=1 cargo test --all

code-rust:
    cd code/rust && npm install && ./node_modules/vsce/out/vsce package
    code --install-extension ./code/rust/fall-rust-0.0.1.vsix

code-fall:
    cd code/fall && npm install && ./node_modules/vsce/out/vsce package
    code --install-extension ./code/fall/fall-fall-0.0.1.vsix
