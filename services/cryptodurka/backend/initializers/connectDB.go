package initializers

import (
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"backend/models"
)

var DB *gorm.DB

func ConnectDB() {
	connectionString := "postgres://root:password@postgres/galoperidolnaya"
	var err error

	DB, err = gorm.Open(postgres.Open(connectionString))
	if err != nil {
		log.Fatal("Failed to connect to the Database")
	}

	DB.Exec("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
	DB.AutoMigrate(&models.User{})
}
