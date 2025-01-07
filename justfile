default:
  just --list

rust:
  cd examples/rust && cargo build

serve:
  mkdocs serve
