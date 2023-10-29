package server

import (
	"encoding/json"
	"errors"
	"klub-zarya/internal/dto"
	"klub-zarya/internal/middleware"
	"klub-zarya/internal/models"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

func (s Server) Register(c *gin.Context) {
	var request dto.RegisterRequest
	if err := json.NewDecoder(c.Request.Body).Decode(&request); err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	if err := s.usecase.Auth.Register(&request); err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrDuplicateUser) {
			httpStatus = http.StatusBadRequest
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, nil)
}

func (s Server) Login(c *gin.Context) {
	var request dto.LoginRequest
	if err := json.NewDecoder(c.Request.Body).Decode(&request); err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	res, err := s.usecase.Auth.Login(&request)
	if err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrWrongCredentials) {
			httpStatus = http.StatusBadRequest
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}

func (s Server) CreateFriendRequest(c *gin.Context) {
	userId := middleware.GetUserId(c)

	targetUserId, err := strconv.ParseInt(c.Query("targetUserId"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	if err := s.usecase.User.CreateFriendRequest(userId, targetUserId); err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrDuplicateUserFriend) {
			httpStatus = http.StatusBadRequest
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusCreated, nil)
}

func (s Server) GetUserFriends(c *gin.Context) {
	userId := middleware.GetUserId(c)

	page, err := strconv.ParseUint(c.Query("page"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: "error with page parameter: " + err.Error(),
		})
		return
	}

	res, err := s.usecase.User.GetUserFriends(userId, page)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}

func (s Server) GetUserFriendRequests(c *gin.Context) {
	userId := middleware.GetUserId(c)

	res, err := s.usecase.User.GetUserFriendRequests(userId)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}

func (s Server) AcceptFriendRequest(c *gin.Context) {
	userId := middleware.GetUserId(c)

	targetUserId, err := strconv.ParseInt(c.Param("targetUserId"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	if err := s.usecase.User.AcceptFriendRequest(userId, targetUserId); err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, nil)
}

func (s Server) CreateTopic(c *gin.Context) {
	userId := middleware.GetUserId(c)

	var request dto.CreateTopicRequest
	if err := json.NewDecoder(c.Request.Body).Decode(&request); err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	res, err := s.usecase.Topic.CreateTopic(userId, &request)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusCreated, res)
}

func (s Server) GetUserTopics(c *gin.Context) {
	userId := middleware.GetUserId(c)

	targetUserId, err := strconv.ParseInt(c.Param("targetUserId"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	page, err := strconv.ParseUint(c.Query("page"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: "error with page parameter: " + err.Error(),
		})
		return
	}

	res, err := s.usecase.Topic.GetUserTopics(userId, targetUserId, page)
	if err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrUserNotFound) {
			httpStatus = http.StatusBadRequest
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}

func (s Server) GetTopics(c *gin.Context) {
	page, err := strconv.ParseUint(c.Query("page"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: "error with page parameter: " + err.Error(),
		})
		return
	}

	res, err := s.usecase.Topic.GetPublicTopics(page)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}

func (s Server) PostComment(c *gin.Context) {
	userId := middleware.GetUserId(c)

	var request dto.PostCommentRequest
	if err := json.NewDecoder(c.Request.Body).Decode(&request); err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}

	if err := s.usecase.Comment.PostComment(userId, &request); err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrAccessDenied) {
			httpStatus = http.StatusForbidden
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusCreated, nil)
}

func (s Server) GetTopicComments(c *gin.Context) {
	userId := middleware.GetUserId(c)

	topicId, err := strconv.ParseInt(c.Param("topicId"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	page, err := strconv.ParseUint(c.Query("page"), 10, 64)
	if err != nil {
		c.JSON(http.StatusInternalServerError, dto.ErrorResponse{
			Message: "error with page parameter: " + err.Error(),
		})
		return
	}

	res, err := s.usecase.Comment.GetTopicComments(userId, topicId, page)
	if err != nil {
		var httpStatus int
		if errors.Is(err, models.ErrAccessDenied) {
			httpStatus = http.StatusForbidden
		} else {
			httpStatus = http.StatusInternalServerError
		}
		c.JSON(httpStatus, dto.ErrorResponse{
			Message: err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, res)
}
