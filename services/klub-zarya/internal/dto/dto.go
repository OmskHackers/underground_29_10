package dto

import "klub-zarya/internal/models"

type RegisterRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginRequest = RegisterRequest

type LoginResponse struct {
	UserId int64 `json:"userId"`
	Token string `json:"token"`
}

type GetFriendsResponse struct {
	Friends []int64 `json:"friends"`
}

type GetFriendsRequestsResponse = GetFriendsResponse

type CreateTopicRequest struct {
	Theme       string `json:"theme"`
	Description string `json:"description"`
	IsPublic    bool   `json:"isPublic"`
}

type CreateTopicResponse struct {
	TopicId int64 `json:"topicId"`
}

type GetTopicsResponse struct {
	Topics []*models.Topic `json:"topics"`
}

type PostCommentRequest struct {
	TopicId int64  `json:"topicId"`
	Text    string `json:"text"`
}

type GetTopicCommentsResponse struct {
	Comments []*models.Comment `json:"comments"`
}

type GetUsersResponse struct {
	Users []*models.User `json:"users"`
}

type ErrorResponse struct {
	Message string `json:"message"`
}
