package db

import (
	"database/sql"

	"github.com/sirupsen/logrus"
	_ "github.com/lib/pq"
)

func GetConnect(cfg *Config, log *logrus.Logger) (*sql.DB, error) {
	log.Infof("Connecting to DB on %s:%d/%s as '%s' ... ", cfg.Host, cfg.Port, cfg.DatabaseName, cfg.User)
	db, err := sql.Open(cfg.DriverName, cfg.DSN())
	if err != nil {
		log.Error(err.Error())
		return nil, err
	}
	err = db.Ping()
	if err != nil {
		log.Error(err.Error())
		return nil, err
	}
	return db, nil
}
