default:
  just --list

test:
  cd tests && ./generate.sh && cargo test

serve:
  mkdocs serve
