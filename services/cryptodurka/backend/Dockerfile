FROM golang:1.20.5-alpine

WORKDIR /app

COPY . .

RUN go mod download

CMD ["go", "run", "."]