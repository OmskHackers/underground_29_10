package server

import (
	"klub-zarya/internal/usecases"

	"github.com/gin-gonic/gin"
)

type Server struct {
	Router *gin.Engine
	listenOn string
	done    chan error
	usecase *usecases.Usecase
}

func NewServer(listenOn string, usecase *usecases.Usecase) *Server {
	router := gin.Default()

	return &Server{
		Router:     router,
		listenOn: listenOn,
		done:    make(chan error),
		usecase: usecase,
	}
}

func (a *Server) Start() error {
	return a.Router.Run(a.listenOn)
}