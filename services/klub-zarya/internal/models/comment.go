package models

import "time"

type Comment struct {
	ID        int64     `json:"id"`
	CreatedAt time.Time `json:"createdAt"`
	Author    string    `json:"author"`
	Text      string    `json:"text"`
}
