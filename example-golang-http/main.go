package main

import (
	"log"
	"net/http"
	_ "net/http/pprof"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("handling request from: %s\n", r.RemoteAddr)
		log.Printf("status: up: %s\n", r.RemoteAddr)
	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}
