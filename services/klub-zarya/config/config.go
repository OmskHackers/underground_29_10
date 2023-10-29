package config

import (
	"klub-zarya/internal/utils"
	"klub-zarya/pkg/auth"
	"klub-zarya/pkg/db"
	"os"
	"strconv"
)

type Config struct {
	Auth *auth.Config
	DB   *db.Config
}

func NewConfig() (*Config, error) {
	authCfg := &auth.Config{
		HashSalt: utils.GenerateRandomString(12),
		Secret:   []byte(utils.GenerateRandomString(12)),
	}

	dbPort, err := strconv.Atoi(os.Getenv("DB_PORT"))
	if err != nil {
		return nil, err
	}
	dbCfg := &db.Config{
		Host:         os.Getenv("DB_HOST"),
		Port:         dbPort,
		DatabaseName: os.Getenv("DB_NAME"),
		User:         os.Getenv("DB_USER"),
		Password:     os.Getenv("DB_PASS"),
		DriverName:   os.Getenv("DB_DRIVER"),
	}

	return &Config{
		Auth: authCfg,
		DB:   dbCfg,
	}, nil
}
