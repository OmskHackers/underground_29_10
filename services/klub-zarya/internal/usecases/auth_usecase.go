package usecases

import (
	"klub-zarya/config"
	"klub-zarya/internal/dto"
	"klub-zarya/internal/models"
	"klub-zarya/internal/repositories"
	"klub-zarya/internal/utils"
	"klub-zarya/pkg/auth"
	"strings"

	"github.com/sirupsen/logrus"
)

type AuthUsecase struct {
	log      *logrus.Logger
	cfg      *config.Config
	jwtAuth  *auth.JwtAuthenticator
	userRepo *repositories.UserRepository
}

func NewAuthUsecase(
	log *logrus.Logger,
	cfg *config.Config,
	jwtAuth *auth.JwtAuthenticator,
	userRepo *repositories.UserRepository,
) *AuthUsecase {
	return &AuthUsecase{
		log,
		cfg,
		jwtAuth,
		userRepo,
	}
}

func (u *AuthUsecase) Register(req *dto.RegisterRequest) error {
	err := u.userRepo.CreateOne(req.Username, utils.CryptString(req.Password, u.cfg.Auth.HashSalt))
	if err != nil {
		if strings.Contains(err.Error(), "duplicate") {
			return models.ErrDuplicateUser
		}
		return err
	}
	u.log.Infof("registered new user %s", req.Username)
	return nil
}

func (u *AuthUsecase) Login(req *dto.LoginRequest) (*dto.LoginResponse, error) {
	user, err := u.userRepo.GetOneByUsername(req.Username)
	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			return nil, models.ErrWrongCredentials
		}
		return nil, err
	}

	if user.Password != utils.CryptString(req.Password, u.cfg.Auth.HashSalt) {
		return nil, models.ErrWrongCredentials
	}

	token, err := u.jwtAuth.GenerateAccessToken(auth.AccessClaims{
		UserID: user.ID,
	})
	if err != nil {
		return nil, err
	}
	u.log.Infof("user %s is authenticated", user.Username)
	return &dto.LoginResponse{
		Token: token,
	}, nil
}
