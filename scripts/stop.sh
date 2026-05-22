#!/bin/bash

pkill -f production-tool
pkill -f pt-backend
docker stop pt-db

wait
stty sane
