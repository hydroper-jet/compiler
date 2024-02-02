# Parsing tests

To test parsing a program producing output to the command line, run:

```
cargo run --bin hydroper_jet_parser_tester -- --source-path tests/parser/Demo.jet
```

To test parsing a program producing output to two files `.ast.json` and `.diag`, run:

```
cargo run --bin hydroper_jet_parser_tester -- --source-path tests/parser/Demo.jet --file-log
```