package auth

import (
	"fmt"

	"github.com/golang-jwt/jwt"
)

type JwtAuthenticator struct {
	conf *Config
}

func NewJwtAuthenticator(conf *Config) JwtAuthenticator {
	return JwtAuthenticator{
		conf: conf,
	}
}

func (j *JwtAuthenticator) GenerateAccessToken(accessClaims AccessClaims) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, accessClaims)
	jwtToken, err := token.SignedString(j.conf.Secret)
	if err != nil {
		return "", fmt.Errorf("can't create jwt token: %w", err)
	}

	return jwtToken, nil
}

func (j *JwtAuthenticator) ParseAccessToken(jwtToken string) (*AccessClaims, error) {
	ac := AccessClaims{}
	token, err := jwt.ParseWithClaims(jwtToken, &ac, func(token *jwt.Token) (interface{}, error) {
		return j.conf.Secret, nil
	})

	if err != nil {
		return nil, err
	}

	if claims, ok := token.Claims.(*AccessClaims); ok && token.Valid {
		return claims, nil
	}
	return nil, err
}
