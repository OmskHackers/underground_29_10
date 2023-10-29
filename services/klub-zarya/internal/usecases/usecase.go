package usecases

type Usecase struct {
	Auth *AuthUsecase
	User *UserUsecase
	Topic *TopicUsecase
	Comment *CommentUsecase
}

func NewUsecase(auth *AuthUsecase, user *UserUsecase, topic *TopicUsecase, comment *CommentUsecase) *Usecase {
	return &Usecase{Auth: auth, User: user, Topic: topic, Comment: comment}
}
