package repositories

import (
	"database/sql"
	"klub-zarya/internal/models"

	sq "github.com/Masterminds/squirrel"
)

type CommentRepository struct {
	conn *sql.DB
}

func NewCommentRepository(conn *sql.DB) *CommentRepository {
	return &CommentRepository{conn: conn}
}

func (r *CommentRepository) CreateOne(authorId, topicId int64, text string) error {
	_, err := sq.Insert("comments").
		Columns("author_id", "topic_id", "text").
		Values(authorId, topicId, text).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		Exec()
	return err
}

func (r *CommentRepository) GetManyByTopicId(topicId int64, paginationIndex uint64) ([]*models.Comment, error) {
	comments := make([]*models.Comment, 0)

	rows, err := sq.Select("id", "u.username", "created_at", "text").
		From("comments").
		OrderBy("created_at DESC").
		Where(sq.Eq{"topic_id": topicId}).
		LeftJoin("users u ON author_id = u.id").
		Limit(50).
		Offset(50 * paginationIndex).
		RunWith(r.conn).
		PlaceholderFormat(sq.Dollar).
		Query()
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		comment := &models.Comment{}

		if err := rows.Scan(&comment.ID, &comment.Author, &comment.CreatedAt, &comment.Text); err != nil {
			return nil, err
		}

		comments = append(comments, comment)
	}

	return comments, nil
}
