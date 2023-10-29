package repositories

import (
	"database/sql"
	"klub-zarya/internal/models"

	sq "github.com/Masterminds/squirrel"
)

type TopicRepository struct {
	conn *sql.DB
}

func NewTopicRepository(conn *sql.DB) *TopicRepository {
	return &TopicRepository{conn: conn}
}

func (r *TopicRepository) CreateOne(userId int64, theme, description string, isPublic bool) error {
	_, err := sq.Insert("topics").
		Columns("theme", "author_id", "description", "is_public").
		Values(theme, userId, description, isPublic).
		PlaceholderFormat(sq.Dollar).
		RunWith(r.conn).
		Exec()
	return err
}

func (r *TopicRepository) GetManyByUser(userId int64, paginationIndex uint64) ([]*models.Topic, error) {
	topics := make([]*models.Topic, 0)

	rows, err := sq.Select("t.id", "t.theme", "u.username", "t.description", "t.is_public", "t.created_at").
		From("topics t").
		OrderBy("created_at DESC").
		LeftJoin("users u ON u.id = author_id").
		Where(sq.Eq{"author_id": userId}).
		Limit(50).
		Offset(50 * paginationIndex).
		PlaceholderFormat(sq.Dollar).
		RunWith(r.conn).
		Query()
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		topic := &models.Topic{}

		if err := rows.Scan(&topic.ID, &topic.Theme, &topic.Author, &topic.Description, &topic.IsPublic, &topic.CreatedAt); err != nil {
			return nil, err
		}

		topics = append(topics, topic)
	}

	return topics, nil
}

func (r *TopicRepository) GetMany(paginationIndex uint64) ([]*models.Topic, error) {
	topics := make([]*models.Topic, 0)

	rows, err := sq.Select("t.id", "t.theme", "u.username", "t.description", "t.is_public", "t.created_at").
		From("topics t").
		OrderBy("created_at DESC").
		LeftJoin("users u ON u.id = author_id").
		Where(sq.Eq{"is_public": true}).
		Limit(50).
		Offset(50 * paginationIndex).
		PlaceholderFormat(sq.Dollar).
		RunWith(r.conn).
		Query()
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		topic := &models.Topic{}

		if err := rows.Scan(&topic.ID, &topic.Theme, &topic.Author, &topic.Description, &topic.IsPublic, &topic.CreatedAt); err != nil {
			return nil, err
		}

		topics = append(topics, topic)
	}

	return topics, nil
}
