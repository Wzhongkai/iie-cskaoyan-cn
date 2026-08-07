#!/usr/bin/env bash
set -euo pipefail

systemctl stop iie-direct-https-test.service || true
systemctl start nginx
