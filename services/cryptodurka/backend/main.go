package main

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func test(c *gin.Context) {
	login := c.PostForm("login")
	password := c.PostForm("password")

	fmt.Println(login, password)

	c.JSON(http.StatusOK, gin.H{
		"Login":    login,
		"Password": password,
	})
}

func main() {
	router := gin.Default()

	router.POST("/api/test", test)

	router.Run(":8081")
}
