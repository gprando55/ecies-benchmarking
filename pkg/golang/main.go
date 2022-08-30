package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"time"

	ecies "github.com/ecies/go/v2"
)

func main() {
	fileName := getFileName()

	file := readFile(fileName)

	key := generateKey()

	start := time.Now()

	ciphertext := encrypt(key, file)

	// _ := decrypt(key, ciphertext)
	decrypt(key, ciphertext)

	elapsed := time.Since(start)

	fmt.Printf("page took %f\n", elapsed.Seconds())
}

func getFileName() string {
	if len(os.Args) < 3 {
		panic("FileName is required, pls running with flag `go run main.go --file filename`")
	}
	return os.Args[2]
}

func encrypt(key *ecies.PrivateKey, file []byte) []byte {
	ciphertext, err := ecies.Encrypt(key.PublicKey, file)

	if err != nil {
		panic(err)
	}

	fmt.Printf("plaintext encrypted: %v\n", ciphertext)

	return ciphertext
}

func decrypt(key *ecies.PrivateKey, ciphertext []byte) []byte {
	plaintext, err := ecies.Decrypt(key, ciphertext)
	if err != nil {
		panic(err)
	}
	fmt.Printf("ciphertext decrypted: %s\n", string(plaintext))

	return plaintext
}

func readFile(fileName string) []byte {
	file, err := ioutil.ReadFile("../../dataset/" + fileName + ".txt")
	if err != nil {
		panic(err)
	}

	fmt.Println("file bytes ", file)

	return file

}

func generateKey() *ecies.PrivateKey {
	k, err := ecies.GenerateKey()
	if err != nil {
		panic(err)
	}

	fmt.Println("key pair has been generated")

	return k
}
