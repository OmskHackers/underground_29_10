package auth

import (
	"context"
	"net/http"

	"github.com/golang-jwt/jwt"
)

type AccessClaims struct {
	UserID             int64  `json:"userId"`
	Token              string `json:"token"`
	jwt.StandardClaims `json:"claims"`
}

type ContextKey string

const UserContextKey ContextKey = "authorized_user"

type Authenticator struct {
}

func NewAuthenticator() *Authenticator {
	return &Authenticator{}
}

func (a *Authenticator) GetToken(request *http.Request) (*string, error) {
	token := request.Header.Get("Authorization")
	if len(token) > 0 {
		return &token, nil
	}

	return nil, nil
}

func (ac AccessClaims) WithContext(ctx context.Context) context.Context {
	return context.WithValue(ctx, UserContextKey, ac)
}
