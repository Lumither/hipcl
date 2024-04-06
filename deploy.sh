#!/usr/bin/bash

cargo build -r
cp ./target/release/loader ./example_env/loader
cp ./target/release/plugin ./example_env/plugin

./example_env/loader

