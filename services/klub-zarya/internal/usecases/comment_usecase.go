package usecases

import (
	"klub-zarya/internal/dto"
	"klub-zarya/internal/models"
	"klub-zarya/internal/repositories"
	"strings"

	"github.com/sirupsen/logrus"
)

type CommentUsecase struct {
	log         *logrus.Logger
	userRepo    *repositories.UserRepository
	commentRepo *repositories.CommentRepository
}

func NewCommentUsecase(log *logrus.Logger, commentRepo *repositories.CommentRepository) *CommentUsecase {
	return &CommentUsecase{
		log:         log,
		commentRepo: commentRepo,
	}
}

func (u *CommentUsecase) PostComment(userId int64, req *dto.PostCommentRequest) error {
	topicAuthor, err := u.userRepo.GetOneByTopicId(req.TopicId)
	if err != nil {
		if strings.Contains(err.Error(), "no rows") {
			return models.ErrUserNotFound
		}
	}
	hasPrivateAccess := true
	if userId != topicAuthor.ID {
		_, err := u.userRepo.GetUserFriendById(topicAuthor.ID, userId)
		if err != nil {
			if strings.Contains(err.Error(), "no rows") {
				hasPrivateAccess = false
			} else {
				return err
			}
		}
	}

	if !hasPrivateAccess {
		return models.ErrAccessDenied
	}
	return u.commentRepo.CreateOne(userId, req.TopicId, req.Text)
}

func (u *CommentUsecase) GetTopicComments(userId, topicId int64, page uint64) (*dto.GetTopicCommentsResponse, error) {
	topicAuthor, err := u.userRepo.GetOneByTopicId(topicId)
	if err != nil {
		if strings.Contains(err.Error(), "no rows") {
			return nil, models.ErrUserNotFound
		}
	}
	hasPrivateAccess := true
	if userId != topicAuthor.ID {
		_, err := u.userRepo.GetUserFriendById(topicAuthor.ID, userId)
		if err != nil {
			if strings.Contains(err.Error(), "no rows") {
				hasPrivateAccess = false
			} else {
				return nil, err
			}
		}
	}
	if !hasPrivateAccess {
		return nil, models.ErrAccessDenied
	}

	comments, err := u.commentRepo.GetManyByTopicId(topicId, page)
	if err != nil {
		return nil, err
	}

	return &dto.GetTopicCommentsResponse{
		Comments: comments,
	}, nil
}
