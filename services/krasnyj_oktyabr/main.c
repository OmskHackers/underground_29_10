#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <pthread.h>
#include <hiredis/hiredis.h>

#define PORT 8080
#define MAX_CLIENTS 10
#define THREAD_POOL_SIZE 4
#define TIMEOUT_SECONDS 60 // Таймаут в секундах

void *handle_client(void *arg);
void *redis_worker(void *arg);

pthread_mutex_t redis_mutex = PTHREAD_MUTEX_INITIALIZER;
redisContext *redis_conn;

int main() {
    int server_fd, new_socket;
    struct sockaddr_in server_addr, client_addr;
    int opt = 1;
    int addrlen = sizeof(server_addr);
    pthread_t threads[MAX_CLIENTS];
    pthread_t thread_pool[THREAD_POOL_SIZE];

    // Создаем подключение к Redis
    redis_conn = redisConnect("localhost", 6379);
    if (redis_conn == NULL || redis_conn->err) {
        fprintf(stderr, "Ошибка подключения к Redis: %s\n", redis_conn->errstr);
        exit(EXIT_FAILURE);
    }

    // Создаем TCP сокет
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // Настраиваем сокет
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
        perror("setsockopt");
        exit(EXIT_FAILURE);
    }
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Привязываем сокет к адресу и порту
    if (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind failed");
        exit(EXIT_FAILURE);
    }

    // Слушаем порт
    if (listen(server_fd, 3) < 0) {
        perror("listen");
        exit(EXIT_FAILURE);
    }

    printf("Сервер слушает порт %d...\n", PORT);

    // Инициализируем пул потоков для Redis
    for (int i = 0; i < THREAD_POOL_SIZE; i++) {
        if (pthread_create(&thread_pool[i], NULL, redis_worker, NULL) < 0) {
            perror("pthread_create");
            exit(EXIT_FAILURE);
        }
    }

    while (1) {
        if ((new_socket = accept(server_fd, (struct sockaddr *)&client_addr, (socklen_t*)&addrlen)) < 0) {
            perror("accept");
            exit(EXIT_FAILURE);
        }

        printf("Новое подключение, создаем поток для клиента.\n");

        // Создаем новый поток для обработки клиента
        pthread_t thread;
        if (pthread_create(&thread, NULL, handle_client, (void*)&new_socket) < 0) {
            perror("pthread_create");
            exit(EXIT_FAILURE);
        }
    }

    return 0;
}

void *handle_client(void *arg) {
    int client_socket = *((int*)arg);
    struct timeval timeout;
    timeout.tv_sec = TIMEOUT_SECONDS;
    timeout.tv_usec = 0;

    // Устанавливаем таймаут для сокета
    if (setsockopt(client_socket, SOL_SOCKET, SO_RCVTIMEO, (char *)&timeout, sizeof(timeout)) < 0) {
        perror("setsockopt");
        close(client_socket);
        pthread_exit(NULL);
    }

    // Остальной код для обработки клиента и взаимодействия с Redis
    // Ваша логика обработки клиентских запросов здесь.

    close(client_socket);
    pthread_exit(NULL);
}

void *redis_worker(void *arg) {
    // Остальной код для обработки Redis-запросов
    // Ваша логика для выполнения операций Redis здесь.
}
