package main

import (
	"klub-zarya/config"
	"klub-zarya/internal/middleware"
	"klub-zarya/internal/repositories"
	"klub-zarya/internal/server"
	"klub-zarya/internal/usecases"
	"klub-zarya/pkg/auth"
	"klub-zarya/pkg/db"
	"klub-zarya/pkg/logger"
	"time"
)

func main() {
	logger.Init()

	log := &logger.Logger
	cfg, err := config.NewConfig()
	if err != nil {
		log.Fatalln(err)
	}

	dbConn, err := db.GetConnect(cfg.DB, log)
	if err != nil {
		log.Warnln("unable to connect to database, retrying in 5sec")
		time.Sleep(5 * time.Second)
		dbConn, err = db.GetConnect(cfg.DB, log)
		if err != nil {
			log.Fatalln(err)
		}
	}

	jwtAuth := auth.NewJwtAuthenticator(cfg.Auth)

	userRepo := repositories.NewUserRepository(dbConn)
	topicRepo := repositories.NewTopicRepository(dbConn)
	commentRepo := repositories.NewCommentRepository(dbConn)

	authUsecase := usecases.NewAuthUsecase(log, cfg, &jwtAuth, userRepo)
	userUsecase := usecases.NewUserUsecase(log, userRepo)
	topicUsecase := usecases.NewTopicUsecase(log, userRepo, topicRepo)
	commentUsecase := usecases.NewCommentUsecase(log, commentRepo)

	usecase := usecases.NewUsecase(authUsecase, userUsecase, topicUsecase, commentUsecase)

	web := server.NewServer(":7000", usecase)
	authGuard := middleware.NewMiddleWare(log, cfg.Auth, &jwtAuth)

	public := web.Router.Group("/")
	public.POST("/login", web.Login)
	public.POST("/register", web.Register)
	public.GET("/topics", web.GetTopics)
	public.GET("/users", web.GetUsers)

	private := web.Router.Group("/")
	private.Use(authGuard.AuthMiddleware())
	private.POST("/topics", web.CreateTopic)
	private.GET("/topics/:targetUserId", web.GetUserTopics)
	private.POST("/comments", web.PostComment)
	private.GET("/comments/:topicId", web.GetTopicComments)
	private.POST("/friends", web.CreateFriendRequest)
	private.POST("/friends/:targetUserId", web.AcceptFriendRequest)
	private.GET("/friends", web.GetUserFriendRequests)

	if err := web.Start(); err != nil {
		log.Fatal(err)
	}
}
