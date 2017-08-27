bootstrap:
    rewrite=bootstrap cargo test --package fall_gen --test cli

generate_parsers:
    rewrite=parsers cargo test --package fall_gen --test cli
    cargo test -p fall_test -p fall_test -p lang_rust -p lang_rust -p lang_json

update-test-data:
    rewrite_test_data=1 cargo test --all

code:
    cd code && npm install && ./node_modules/vsce/out/vsce package
