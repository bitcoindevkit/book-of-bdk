default:
  just --list

alias t := test

test:
  cd tests && ./generate.sh && cargo test
