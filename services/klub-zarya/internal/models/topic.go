package models

import "time"

type Topic struct {
	ID          int64      `json:"id"`
	CreatedAt   time.Time  `json:"createdAt"`
	Author      string     `json:"author"`
	Theme       string     `json:"theme"`
	IsPublic    bool       `json:"isPublic"`
	Description string     `json:"description"`
	Comments    []*Comment `json:"comments"`
}
