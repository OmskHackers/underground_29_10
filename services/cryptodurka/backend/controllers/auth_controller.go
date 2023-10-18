package controllers

import (
	"backend/models"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type AuthController struct {
	DB *gorm.DB
}

func NewAuthController(DB *gorm.DB) AuthController {
	return AuthController{DB}
}

func (ac *AuthController) Register(ctx *gin.Context) {
	var payload *models.RegisterRequest

	if err := ctx.ShouldBindJSON(&payload); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"status": "fail", "message": err.Error()})
		return
	}

	newUser := models.User{
		Username: payload.Username,
		Password: payload.Password,
	}

	result := ac.DB.Create(&newUser)

	if result.Error != nil && strings.Contains(result.Error.Error(), "duplicate key value violates unique") {
		ctx.JSON(http.StatusConflict, gin.H{"status": "fail", "message": "User with that username already exists"})
		return
	}

	userResponse := &models.UserResponse{
		Username: newUser.Username,
	}

	ctx.JSON(http.StatusCreated, gin.H{"status": "success", "message": gin.H{"user": userResponse}})
}

func (ac *AuthController) Login(ctx *gin.Context) {
	var payload *models.LoginRequest

	if err := ctx.ShouldBindJSON(&payload); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"status": "fail", "message": err.Error()})
		return
	}

	newUser := models.User{
		Username: payload.Username,
		Password: payload.Password,
	}

	result := ac.DB.Create(&newUser)

	if result.Error != nil && strings.Contains(result.Error.Error(), "duplicate key value violates unique") {
		ctx.JSON(http.StatusConflict, gin.H{"status": "fail", "message": "User with that username already exists"})
		return
	}

	userResponse := &models.UserResponse{
		Username: newUser.Username,
	}

	ctx.JSON(http.StatusCreated, gin.H{"status": "success", "message": gin.H{"user": userResponse}})
}
