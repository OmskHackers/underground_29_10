package utils

import (
	"crypto/sha1"
	"fmt"
	"math/rand"
)

const dict string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

func CryptString(payload string, salt string) string {
	pwd := sha1.New()
	pwd.Write([]byte(payload))
	pwd.Write([]byte(salt))
	return fmt.Sprintf("%x", pwd.Sum(nil))
}

func GenerateRandomString(size int) string {
	res := ""
	for i := 0; i < size; i++ {
		res += fmt.Sprint(dict[rand.Intn(len(dict)-1)])
	}
	return res
}
