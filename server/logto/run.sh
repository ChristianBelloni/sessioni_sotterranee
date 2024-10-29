#!/bin/bash

npm run start &

nginx &

wait

exit $?
