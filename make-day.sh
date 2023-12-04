#!/bin/bash

function make_day() {
  day=$(printf "%02d" "$1")
  part=$2

  name="day${day}${part}"

  cargo new "$name"
  touch "$name/input.txt"
  touch "$name/example.txt"

  cp "template/src/main.rs" "$name/src/main.rs"
}

make_day "$1" "a"
make_day "$1" "b"



