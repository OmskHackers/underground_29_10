#!/bin/bash

sqlite3 zapiski.db <<'END_SQL'
.read zapiski_db.sql
END_SQL
