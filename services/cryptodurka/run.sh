#!/bin/sh
chown -R cryptopsycho:2000 ./data
su cryptopsycho -c "touch data/sqlite.db"
su cryptopsycho -c "sqlite3 data/sqlite.db < init.sql"
su cryptopsycho -c "PYTHONUNBUFFERED=1 python3 /service/main.py"