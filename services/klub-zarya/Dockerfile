FROM golang:1.21.3-alpine

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download && go mod verify

COPY . .

RUN go build -o main ./cmd/main.go

ENTRYPOINT [ "./main" ]