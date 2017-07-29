generate-parsers:
    rewrite=1 cargo test --package fall_gen

update-test-data:
    rewrite_test_data=1 cargo test --all
