default:
  just --list

alias t := test
alias s := serve

test:
  cd tests && ./generate.sh && cargo test

serve:
  mkdocs serve