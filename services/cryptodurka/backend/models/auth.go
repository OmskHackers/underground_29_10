package models

type LoginRequest struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
}

type RegisterRequest struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
	AboutMe string `json:"aboutme"`
	IsPrivate  bool   `json:"isprivate"`
}

type ProfileResponse struct {
	Username string `json:"username,omitempty"`
	IsPrivate  bool   `json:"isprivate,omitempty"`
}