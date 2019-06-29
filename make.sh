#!/bin/bash
rm main
rustc  -C opt-level=3 main.rs
./main
