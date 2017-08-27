test:
    cargo test --all

bootstrap:
    rewrite=bootstrap cargo test --package fall_gen --test cli
    cargo test --all

generate-parsers:
    rewrite=parsers cargo test --package fall_gen --test cli
    cargo test -p fall_test -p fall_test -p lang_rust -p lang_rust -p lang_json

update-test-data:
    rewrite_test_data=1 cargo test --all

code:
    cd code && npm install && ./node_modules/vsce/out/vsce package
    code --install-extension ./code/fall-code-0.0.1.vsix
