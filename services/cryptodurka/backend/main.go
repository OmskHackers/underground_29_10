package main

import (
	"backend/controllers"
	"backend/initializers"
	"backend/routes"

	"github.com/gin-gonic/gin"
)

var (
	server              *gin.Engine
	AuthController      controllers.AuthController
	AuthRouteController routes.AuthRouteController
)

func init() {
	initializers.ConnectDB()

	AuthController = controllers.NewAuthController(initializers.DB)
	AuthRouteController = routes.NewAuthRouteController(AuthController)

	server = gin.Default()
}

func main() {
	router := server.Group("/api")
	AuthRouteController.AuthRoute(router)
	server.Run(":8081")
}
