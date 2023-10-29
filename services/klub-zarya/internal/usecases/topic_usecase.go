package usecases

import (
	"fmt"
	"klub-zarya/internal/dto"
	"klub-zarya/internal/models"
	"klub-zarya/internal/repositories"
	"strings"

	"github.com/sirupsen/logrus"
)

type TopicUsecase struct {
	log       *logrus.Logger
	userRepo  *repositories.UserRepository
	topicRepo *repositories.TopicRepository
}

func NewTopicUsecase(log *logrus.Logger, userRepo *repositories.UserRepository, topicRepo *repositories.TopicRepository) *TopicUsecase {
	return &TopicUsecase{
		log,
		userRepo,
		topicRepo,
	}
}

func (u *TopicUsecase) CreateTopic(userId int64, req *dto.CreateTopicRequest) (*dto.CreateTopicResponse, error) {
	id, err := u.topicRepo.CreateOne(userId, req.Theme, req.Description, req.IsPublic)
	if err != nil {
		return nil, err
	}
	return &dto.CreateTopicResponse{
		TopicId: id,
	}, nil
}

func (u *TopicUsecase) GetUserTopics(userId, targetUserId int64, page uint64) (*dto.GetTopicsResponse, error) {
	hasPrivateAccess := true

	if userId != targetUserId {
		_, err := u.userRepo.GetUserFriendById(targetUserId, userId)
		if err != nil {
			if strings.Contains(err.Error(), "no rows") {
				hasPrivateAccess = false
			} else {
				return nil, err
			}
		}
	}
	fmt.Println(hasPrivateAccess)

	topics, err := u.topicRepo.GetManyByUser(targetUserId, page)
	if err != nil {
		return nil, err
	}

	filteredTopics := make([]*models.Topic, 0)
	if !hasPrivateAccess {
		for _, topic := range topics {
			if topic.IsPublic {
				filteredTopics = append(filteredTopics, topic)
			}
		}
	} else {
		filteredTopics = topics
	}

	return &dto.GetTopicsResponse{
		Topics: filteredTopics,
	}, nil
}

func (u *TopicUsecase) GetPublicTopics(page uint64) (*dto.GetTopicsResponse, error) {
	fmt.Println(page)
	topics, err := u.topicRepo.GetMany(page)
	if err != nil {
		return nil, err
	}
	return &dto.GetTopicsResponse{
		Topics: topics,
	}, nil
}
