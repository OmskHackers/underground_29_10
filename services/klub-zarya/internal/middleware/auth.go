package middleware

import (
	"context"
	"klub-zarya/internal/dto"
	"klub-zarya/pkg/auth"
	"net/http"
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
			c.JSON(http.StatusUnauthorized, dto.ErrorResponse {
				Message: "malformed token",
			})
			return
		}
		jwtToken := authHeader[1]

		claims, err := m.jwtAuth.ParseAccessToken(jwtToken)

		if err != nil {
			m.log.Errorf("[AuthGuard] Error :%s", err.Error())
			c.JSON(http.StatusUnauthorized, dto.ErrorResponse {
				Message: "unauthorized",
			})
			return
		}

		c.Request = c.Request.WithContext(context.WithValue(c.Request.Context(), auth.UserContextKey, claims.UserID))
		c.Next()
	}
}

func GetUserIdFromContext(c *gin.Context) int64 {
	claims, ok := c.Request.Context().Value(auth.UserContextKey).(auth.AccessClaims)

	if !ok {
		return 0
	}

	return claims.UserID
}
