#!/bin/bash

run() {
  cyan="\033[0;36m"
  reset="\033[m"

  echo -e "\n"\$ $cyan$@$reset;
  $@;
}

set -e

run rustc $1.rs
run ./$1
run rm $1
