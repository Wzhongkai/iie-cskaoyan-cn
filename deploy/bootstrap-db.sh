#!/usr/bin/env bash
set -euo pipefail

install -d -m 700 /etc/iie-cskaoyan

db_password="$(openssl rand -hex 24)"
admin_token="$(openssl rand -hex 32)"

if ! runuser -u postgres -- psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='iie_app'" | grep -q 1; then
  runuser -u postgres -- createuser iie_app
fi
runuser -u postgres -- psql -c "ALTER ROLE iie_app WITH LOGIN PASSWORD '${db_password}';"

if ! runuser -u postgres -- psql -tAc "SELECT 1 FROM pg_database WHERE datname='iie'" | grep -q 1; then
  runuser -u postgres -- createdb -O iie_app iie
fi

umask 077
printf 'DATABASE_URL=postgres://iie_app:%s@127.0.0.1/iie\nAPI_BIND=127.0.0.1:9000\nADMIN_TOKEN=%s\nRUST_LOG=info\n' \
  "$db_password" "$admin_token" > /etc/iie-cskaoyan/api.env
printf 'UPLOAD_DIR=/srv/iie-cskaoyan/uploads\n' >> /etc/iie-cskaoyan/api.env
install -d -o deploy -g deploy -m 750 /srv/iie-cskaoyan/uploads
printf 'API_INTERNAL_URL=http://127.0.0.1:9000\n' > /etc/iie-cskaoyan/web.env
chown root:root /etc/iie-cskaoyan/api.env /etc/iie-cskaoyan/web.env
chmod 600 /etc/iie-cskaoyan/api.env /etc/iie-cskaoyan/web.env

runuser -u postgres -- psql -d iie -tAc 'SELECT current_database()'
