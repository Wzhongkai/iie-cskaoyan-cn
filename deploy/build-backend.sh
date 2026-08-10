#!/usr/bin/env bash
set -euo pipefail

chown -R deploy:deploy /srv/iie-cskaoyan/backend
runuser -u deploy -- bash -c 'cd /srv/iie-cskaoyan/backend && cargo build --release --locked'
chmod 755 /srv/iie-cskaoyan/backend/target/release/iie-cskaoyan-api
