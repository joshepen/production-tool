#!/bin/bash

docker compose up -d
sleep 3 # Because nothing else I try is working
(cd backend && ./target/debug/pt-backend) &
(cd frontend && npm run dev) &
