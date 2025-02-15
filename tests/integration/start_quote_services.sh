#!/bin/bash


cargo build

# Run quote_api in the background
nohup cargo run -p quote_api > quote_api.log 2>&1 &
echo $! > quote_api.pid
echo "quote_api is running in the background (PID: $(cat quote_api.pid))"
sleep 5

# Run quote_sinker in the background
nohup cargo run -p quote_sinker > quote_sinker.log 2>&1 &
echo $! > quote_sinker.pid
echo "quote_sinker is running in the background (PID: $(cat quote_sinker.pid))"
