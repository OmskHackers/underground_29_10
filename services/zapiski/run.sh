#!/bin/sh
chown -R pipiski:2000 ./data
su pipiski -c "touch data/sqlite.db"
su pipiski -c "sqlite3 data/sqlite.db < zapiski_db.sql"
su pipiski -c "PYTHONUNBUFFERED=1 python3 /service/zapiski.py"
