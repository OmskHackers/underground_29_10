package middleware

import (
	"fmt"
	"klub-zarya/internal/dto"
	"klub-zarya/pkg/auth"
	"net/http"
	"strconv"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
)

type Middleware struct {
	jwtAuth *auth.JwtAuthenticator
	log     *logrus.Logger
	cfg     *auth.Config
}

func NewMiddleWare(log *logrus.Logger, cfg *auth.Config, jwtAuth *auth.JwtAuthenticator) *Middleware {
	return &Middleware{
		jwtAuth,
		log,
		cfg,
	}
}

func (m *Middleware) AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		authHeader := strings.Split(c.Request.Header.Get("Authorization"), "Bearer ")
		if len(authHeader) != 2 {
			c.JSON(http.StatusUnauthorized, dto.ErrorResponse{
				Message: "malformed token",
			})
			return
		}
		jwtToken := authHeader[1]

		claims, err := m.jwtAuth.ParseAccessToken(jwtToken)

		if err != nil {
			m.log.Errorf("[AuthGuard] Error :%s", err.Error())
			c.JSON(http.StatusUnauthorized, dto.ErrorResponse{
				Message: "unauthorized",
			})
			return
		}
		c.AddParam("userId", fmt.Sprint(claims.UserID))
		c.Next()
	}
}

func GetUserId(c *gin.Context) int64 {
	userId, err := strconv.ParseInt(c.Param("userId"), 10, 64)
	if err != nil {
		return 0
	}

	return userId
}
