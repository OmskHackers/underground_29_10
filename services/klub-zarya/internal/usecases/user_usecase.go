package usecases

import (
	"klub-zarya/internal/dto"
	"klub-zarya/internal/models"
	"klub-zarya/internal/repositories"
	"strings"

	"github.com/sirupsen/logrus"
)

type UserUsecase struct {
	log      *logrus.Logger
	userRepo *repositories.UserRepository
}

func NewUserUsecase(log *logrus.Logger, userRepo *repositories.UserRepository) *UserUsecase {
	return &UserUsecase{
		log,
		userRepo,
	}
}

func (u *UserUsecase) GetUserFriends(userId int64, page uint64) (*dto.GetFriendsResponse, error) {
	friends, err := u.userRepo.GetUserFriends(userId, page)
	if err != nil {
		return nil, err
	}

	friendsUsernames := make([]string, 0, len(friends))
	for _, friend := range friends {
		friendsUsernames = append(friendsUsernames, friend.Username)
	}

	return &dto.GetFriendsResponse{
		Friends: friendsUsernames,
	}, nil
}

func (u *UserUsecase) GetUserFriendRequests(userId int64) (*dto.GetFriendsRequestsResponse, error) {
	requests, err := u.userRepo.GetUnconfirmedUserFriends(userId)
	if err != nil {
		return nil, err
	}
	return &dto.GetFriendsRequestsResponse{
		Friends: requests,
	}, nil
}

func (u *UserUsecase) CreateFriendRequest(userId, targetUserId int64) error {
	if err := u.userRepo.CreateUserFriend(userId, targetUserId, false); err != nil {
		if strings.Contains(err.Error(), "duplicate") {
			return models.ErrDuplicateUserFriend
		}
		return err
	}
	return nil
}

func (u *UserUsecase) AcceptFriendRequest(userId, fromUserId int64) error {
	_, err := u.userRepo.GetUnconfirmedUserFriends(userId)
	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			return models.ErrFriendsRequestsNotFound
		}
		return err
	}
	return u.userRepo.UpdateUserFriend(userId, fromUserId)
}