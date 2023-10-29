package models

import "errors"

var (
	ErrUserNotFound        = errors.New("искомый товарищ не обнаружен")
	ErrDuplicateUser       = errors.New("товарищ уже является членом клуба")
	ErrWrongCredentials    = errors.New("неверные данные для входа")
	ErrDuplicateUserFriend = errors.New("товарищ уже у вас в друзьях или вы уже подавали заявку к нему в друзья")
	ErrAccessDenied        = errors.New("товарищ! проход запрещьон!")
	ErrFriendsRequestsNotFound = errors.New("у вас нет запросов в друзья")
)
