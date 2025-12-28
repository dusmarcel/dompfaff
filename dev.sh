#!/bin/sh
set -e

tailwindcss \
  -i ./packages/ui/input.css \
  -o ./packages/ui/assets/output.css \
  --watch &

tailwindcss \
  -i ./packages/web/input.css \
  -o ./packages/web/assets/output.css \
  --watch &

exec dx serve --package web --addr 0.0.0.0