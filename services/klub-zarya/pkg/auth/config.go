package auth

type Config struct {
	Secret   []byte `json:"secret"`
	HashSalt string `json:"hashSalt"`
}
