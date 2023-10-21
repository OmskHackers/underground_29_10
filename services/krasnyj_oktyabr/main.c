#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h> 
#include <pthread.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <hiredis/hiredis.h>
#include <fcntl.h>

#define MAX_THREADS 10000
#define TIMEOUT 120

int random_fd;
pthread_t threads[MAX_THREADS];

typedef struct {
    int client_socket;
    redisContext *redis;
} client_data;

char *create_fake_flag() {
    const char *BASE36_ARGS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const int length = strlen(BASE36_ARGS);
    char *result = (char *) calloc(34, sizeof(char));
    uint64_t data = 0;

    for (int i = 0; i < 31; ++i) {
        read(random_fd, &data, sizeof(data));
        result[i] = BASE36_ARGS[data % length];
    }
    result[31] = '=';
    result[32] = '\n';

    return result;
}

void handle_register(int client_socket, const char *login, const char *pass) {
    // Обработка регистрации

    const char *response = "handle_register ОК\n";
    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

    if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
    }
}

void handle_bake(int client_socket, const char *code, const char *secret_recipe) {
    // Обработка /BAKE

    const char *response = "handle_bake ОК\n";
    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

    if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
    }
}

void handle_get(int client_socket, const char *code) {
    // Ваш код обработки ...
    
    // После успешной обработки отправляем "ОК" клиенту
    const char *response = "handle_get ОК\n";
    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

    if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
    }
}

void handle_admin(int client_socket, const char *code) {
    // Ваш код обработки ...
    
    // После успешной обработки отправляем "ОК" клиенту
    const char *response = "handle_admin ОК\n";
    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

    if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
    }
}


void* handle_client(void* arg) {
    //fprintf(stderr, "connect.\n");
    client_data *data = (client_data*)arg;
    char buffer[1024];

    // Set timeout for client socket
    struct timeval tv;
    tv.tv_sec = TIMEOUT;
    tv.tv_usec = 0;
    setsockopt(data->client_socket, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);

    char *msg = create_fake_flag();
    char greet[] = "\n/REGISTER <login> <pass>\n/BAKE <code> <secret_recipe>\n/GET <code>\n";
    size_t total_length = strlen(msg) + strlen(greet) + 1;
    char *result = malloc(total_length);

    if (result == NULL) {
        fprintf(stderr, "Ошибка выделения памяти.\n");
        return NULL;
    }

    strcpy(result, msg);
    free(msg);
    strcat(result, greet);


    send(data->client_socket, result, total_length - 1, 0);
    //fprintf(stderr, "send greet\n");

    char login[256];
    char password[256];
    char code[256];
    char secret_recipe[256];

    while (1) {
        ssize_t bytes_received = recv(data->client_socket, buffer, sizeof(buffer) - 1, 0);
        if (bytes_received < 0) {
            // Error occurred
            perror("recv error"); // This will print the error description
            break;
        } else if (bytes_received == 0) {
            // Connection closed by client
            break;
        } else {
            buffer[bytes_received] = '\0'; // Null-terminate for safety if you later treat it as a string
        }

        // Now, check if the connection is still alive before sending
        //ssize_t bytes_sent = send(data->client_socket, buffer, bytes_received, 0);
        if (strncmp(buffer, "/REGISTER ", 10) == 0) {
            char login[256], pass[256];
            sscanf(buffer + 10, "%255s %255s", login, pass);
            handle_register(data->client_socket, login, pass);
        }
        else if (strncmp(buffer, "/BAKE ", 6) == 0) {
            char code[256], secret_recipe[256];
            sscanf(buffer + 6, "%255s %255s", code, secret_recipe);
            handle_bake(data->client_socket, code, secret_recipe);
        }
        else if (strncmp(buffer, "/GET ", 5) == 0) {
            char code[256];
            sscanf(buffer + 5, "%255s", code);
            handle_admin(data->client_socket, code);
        } else if (strncmp(buffer, "/ADMIN ", 6) == 0) {
            char code[256];
            sscanf(buffer + 5, "%255s", code);
            handle_admin(data->client_socket, code);
        }else {
            const char *error_message = "Неверный запрос\n";
            ssize_t bytes_sent = send(data->client_socket, error_message, strlen(error_message), 0);

            if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
                break;
            }
        }

    }


    close(data->client_socket);
    redisFree(data->redis);  // Close the Redis connection
    free(data);
    return NULL;
}


int main() {

    signal(SIGPIPE, SIG_IGN);

    int server_socket;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    random_fd = open("/dev/urandom", O_RDONLY);
    

    // Initialize server socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(8080);

    bind(server_socket, (struct sockaddr*)&server_addr, sizeof(server_addr));
    listen(server_socket, 5);

    while (1) {
        int client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_len);

        client_data *c_data = malloc(sizeof(client_data));
        c_data->client_socket = client_socket;
        c_data->redis = redisConnect("redis", 6379);

        if (c_data->redis == NULL || c_data->redis->err) {
            if (c_data->redis) {
                printf("Error connecting to Redis: %s\n", c_data->redis->errstr);
                redisFree(c_data->redis);
            } else {
                printf("Cannot allocate redis context\n");
            }
            close(client_socket);
            free(c_data);
            continue;
        }

        pthread_t thread;
        pthread_create(&thread, NULL, handle_client, c_data);
        pthread_detach(thread);  // The system will automatically reclaim resources when the thread terminates
    }

    close(server_socket);
    return 0;
}
