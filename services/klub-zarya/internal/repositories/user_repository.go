package repositories

import (
	"database/sql"
	"klub-zarya/internal/models"

	sq "github.com/Masterminds/squirrel"
)

type UserRepository struct {
	conn *sql.DB
}

func NewUserRepository(conn *sql.DB) *UserRepository {
	return &UserRepository{conn: conn}
}

func (r *UserRepository) CreateOne(username, password string) error {
	_, err := sq.Insert("users").
		Columns("username", "password").
		Values(username, password).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		Exec()
	if err != nil {
		return err
	}

	return nil
}

func (r *UserRepository) GetOneByUsername(username string) (*models.User, error) {
	user := &models.User{}

	row := sq.Select("id", "username", "password").
		From("users").
		Where(sq.Eq{"username": username}).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		QueryRow()
	if err := row.Scan(&user.ID, &user.Username, &user.Password); err != nil {
		return nil, err
	}
	return user, nil
}

func (r *UserRepository) GetUserFriends(userId int64, paginationIndex uint64) ([]*models.User, error) {
	friends := make([]*models.User, 0)

	rows, err := sq.Select("username").
		From("users_friends uf").
		LeftJoin("users u ON u.id = uf.friend_id").
		Where(sq.And{
			sq.Eq{"uf.user_id": userId},
			sq.Eq{"is_confirmed": true},
		}).
		Limit(50).
		Offset(50 * paginationIndex).
		PlaceholderFormat(sq.Dollar).
		RunWith(r.conn).
		Query()
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		user := &models.User{}

		if err := rows.Scan(&user.Username); err != nil {
			return nil, err
		}

		friends = append(friends, user)
	}
	return friends, nil
}

func (r *UserRepository) CreateUserFriend(userId, friendId int64, isConfirmed bool) error {
	tx, err := r.conn.Begin()
	if err != nil {
		return err
	}

	_, err = sq.Insert("users_friends").
		Columns("user_id", "friend_id", "is_confirmed").
		Values(userId, friendId, isConfirmed).
		RunWith(tx).
		PlaceholderFormat(sq.Dollar).
		Exec()
	if err != nil {
		if err := tx.Rollback(); err != nil {
			return err
		}
		return err
	}
	_, err = sq.Insert("users_friends").
		Columns("user_id", "friend_id", "is_confirmed").
		Values(friendId, userId, false).
		RunWith(tx).
		PlaceholderFormat(sq.Dollar).
		Exec()
	if err != nil {
		if err := tx.Rollback(); err != nil {
			return err
		}
		return err
	}
	return tx.Commit()
}

func (r *UserRepository) UpdateUserFriend(userId, friendId int64) error {
	tx, err := r.conn.Begin()
	if err != nil {
		return err
	}

	_, err = sq.Update("users_friends").
		Set("is_confirmed", true).
		Where(sq.And{
			sq.Eq{"user_id": userId},
			sq.Eq{"friend_id": friendId},
		}).
		RunWith(tx).
		PlaceholderFormat(sq.Dollar).
		Exec()
	if err != nil {
		if err := tx.Rollback(); err != nil {
			return err
		}
		return err
	}
	_, err = sq.Update("users_friends").
		Set("is_confirmed", true).
		Where(sq.And{
			sq.Eq{"user_id": friendId},
			sq.Eq{"friend_id": userId},
		}).
		RunWith(tx).
		PlaceholderFormat(sq.Dollar).
		Exec()
	if err != nil {
		if err := tx.Rollback(); err != nil {
			return err
		}
		return err
	}
	return tx.Commit()
}

func (r *UserRepository) GetUserFriendById(userId, friendId int64) (*models.User, error) {
	friend := &models.User{}

	row := sq.Select("u.id", "u.username").
		From("users_friends").
		LeftJoin("users u ON u.id = friend_id").
		Where(sq.Eq{"user_id": userId}).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		QueryRow()
	if err := row.Scan(&friend.ID, &friend.Username); err != nil {
		return nil, err
	}
	return friend, nil
}

func (r *UserRepository) GetOneByTopicId(topicId int64) (*models.User, error) {
	user := &models.User{}

	row := sq.Select("u.id", "u.username").
		From("topics t").
		LeftJoin("users u ON u.id = t.author_id").
		Where(sq.Eq{"t.id": topicId}).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		QueryRow()
	if err := row.Scan(&user.ID, &user.Username); err != nil {
		return nil, err
	}
	return user, nil
}

func (r *UserRepository) GetUnconfirmedUserFriends(userId int64) ([]string, error) {
	friends := make([]string, 0)

	rows, err := sq.Select("u.username").
		From("users_friends").
		LeftJoin("users u ON u.id = friend_id").
		Where(sq.And{
			sq.Eq{"is_confirmed": true},
		}).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		Query()
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		var username string

		if err := rows.Scan(&username); err != nil {
			return nil, err
		}
		friends = append(friends, username)
	}
	return friends, nil
}