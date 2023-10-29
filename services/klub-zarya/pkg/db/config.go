package db

import "fmt"

type Config struct {
	Host         string
	Port         int
	DatabaseName string
	User         string
	Password     string
	DriverName   string
}

func (db *Config) String() string {
	return fmt.Sprintf("Connecting to DB on %s:%d/%s as '%s' ...", db.Host, db.Port, db.DatabaseName, db.User)
}

func (db *Config) DSN() string {
	dsn := fmt.Sprintf(
		"host=%s port=%d dbname=%s sslmode=disable user=%s password=%s",
		db.Host, db.Port, db.DatabaseName, db.User, db.Password,
	)
	return dsn
}
