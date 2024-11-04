#!/bin/sh

rm ./api_specs.json

cargo build

cargo run &

last_pid=$!

sleep 3

curl -s http://localhost:8080/docs/private/api.json | jq . > api_specs.json
sleep 1

kill -KILL $last_pid
