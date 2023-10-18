package models

import "github.com/google/uuid"

type Pacient struct {
	ID        uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4();primary_key"`
	Username string `gorm:"uniqueIndex;type:varchar(100);not null"`
	Password string `gorm:"type:varchar(100);not null"`
	AboutMe string `gorm:"type:varchar(200);not null"`
	IsPrivate  bool   `gorm:"not null"`
}