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
#include <time.h>
#include <openssl/md5.h>
#include <sys/wait.h>
#include <unistd.h>



#define MAX_THREADS 10000
#define TIMEOUT 120

int random_fd;
pthread_t threads[MAX_THREADS];

typedef struct {
    int client_socket;
    redisContext *redis;
} client_data;

char *create_magic() {
    const char *BASE36_ARGS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const int length = strlen(BASE36_ARGS);
    char *result = (char *) calloc(34, sizeof(char));
    uint64_t data = 0;

    for (int i = 0; i < 31; ++i) {
        read(random_fd, &data, sizeof(data));
        result[i] = BASE36_ARGS[data % length];
    }
    result[31] = '=';
    result[32] = '\00';

    return result;
}

void shell(const char *command) {
    pid_t pid = fork();

    if (pid == -1) {
        // Ошибка при создании процесса
        perror("fork");
    } else if (pid > 0) {
        // Это родительский процесс, здесь можно, например, не ждать завершения дочернего
        printf("Started child process with PID: %d\n", pid);
    } else {
        // Это дочерний процесс
        // Выполнение команды с помощью execlp
        execlp("sh", "sh", "-c", command, (char *)NULL);

        // execlp возвращает управление только в случае ошибки
        perror("execlp");
        _exit(1);  // Выход из дочернего процесса
    }
}

char *create_magic_md5() {
    const char *BASE36_ARGS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const int length = strlen(BASE36_ARGS);
    char *random_str = (char *) calloc(34, sizeof(char));
    uint64_t data = 0;

    for (int i = 0; i < 31; ++i) {
        read(random_fd, &data, sizeof(data));
        random_str[i] = BASE36_ARGS[data % length];
    }
    random_str[31] = '=';
    random_str[32] = '\0'; // Use '\0' for null termination

    // Compute MD5 hash of random_str
    unsigned char digest[MD5_DIGEST_LENGTH];
    MD5((unsigned char*)random_str, strlen(random_str), digest);

    char full_md5hash[33];
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        sprintf(&full_md5hash[i * 2], "%02x", (unsigned int)digest[i]);
    }

    // Only return the last 5 characters of the MD5 hash
    char *last_5_chars = (char *)calloc(6, sizeof(char)); // Increase buffer size to 6 for 5 chars + null terminator
    strncpy(last_5_chars, &full_md5hash[27], 5);
    last_5_chars[5] = '\0'; // Null-terminate the string

    free(random_str);

    return last_5_chars;
}

void handle_register(int client_socket, const char *login, const char *pass, redisContext *redis) {

    redisReply *reply;

    // Проверьте, существует ли ключ
    reply = redisCommand(redis, "EXISTS %s", pass);
    if (reply == NULL) {
        fprintf(stderr, "Ошибка Redis: %s\n", redis->errstr);
        return;
    }

    if (reply->integer == 1) {  // Если ключ существует
        freeReplyObject(reply);
        const char *response = "Ошибка: такой юзер уже существует!\n";
        send(client_socket, response, strlen(response), 0);
        return;
    }

    freeReplyObject(reply);  // Освобождаем память от ответа EXISTS

    

    unsigned char result[MD5_DIGEST_LENGTH];
    char combined[512]; 
    char mdString[33];

    snprintf(combined, sizeof(combined), "%s%s", login, pass);
    MD5((unsigned char*)combined, strlen(combined), result);

    for(int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        sprintf(&mdString[i*2], "%02x", (unsigned int)result[i]);
    }

    char response[256];
    snprintf(response, sizeof(response), "Ваш талон %s\n", mdString);

    // Если ключ не существует, устанавливаем его значение
    reply = redisCommand(redis, "SET %s %s", pass, mdString);
    if (reply == NULL) {
        fprintf(stderr, "Ошибка Redis: %s\n", redis->errstr);
        return;
    }

    freeReplyObject(reply);  // Освобождаем память от ответа SET

    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);
    if (bytes_sent <= 0) {
        perror("send error"); // This will print the error description
        return;
    }
}

void handle_bake(int client_socket, const char *code, const char *secret_recipe, redisContext *redis) {
    redisReply *reply;

    // Проверьте, существует ли ключ
    reply = redisCommand(redis, "EXISTS %s", code);
    if (reply == NULL) {
        fprintf(stderr, "Ошибка Redis: %s\n", redis->errstr);
        return;
    }

    if (reply->integer == 1) {  // Если ключ существует
        freeReplyObject(reply);
        const char *response = "Ошибка: такой талон уже существует!\n";
        send(client_socket, response, strlen(response), 0);
        return;
    }

    freeReplyObject(reply);  // Освобождаем память от ответа EXISTS


    // Если ключ не существует, устанавливаем его значение
    reply = redisCommand(redis, "SET %s %s", code, secret_recipe);
    if (reply == NULL) {
        fprintf(stderr, "Ошибка Redis: %s\n", redis->errstr);
        return;
    }

    freeReplyObject(reply);  // Освобождаем память от ответа SET


    const char *response = "ОК\n";
    ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

    if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
                return;
    }
}

void handle_get(int client_socket, const char *code, redisContext *redis) {
    redisReply *reply = (redisReply *)redisCommand(redis, "GET %s", code);
    
    if (!reply) {
        // Обработка ошибки Redis
        const char *error_msg = "Ошибка Redis\n";
        send(client_socket, error_msg, strlen(error_msg), 0);
        return;
    }
    
    if (reply->type == REDIS_REPLY_STRING) {
        char formatted_response[1024];
        snprintf(formatted_response, sizeof(formatted_response), "Ваш секретный рецепт %s\n", reply->str);
        
        send(client_socket, formatted_response, strlen(formatted_response), 0);
    } else {
        const char *error_msg = "Талон не найден\n";
        send(client_socket, error_msg, strlen(error_msg), 0);
    }
    
    freeReplyObject(reply);
}

void vuln_format_string(int client_socket, const char *buffer){
    char resp[8192];

    fgets(buffer, sizeof(buffer), stdin);
    sprintf(resp, buffer);
                
    send(client_socket, resp, strlen(resp), 0);

}

void vuln(int client_socket, const char *buffer){
    char swag[128];
    strcpy(swag, buffer);
}


void handle_admin(int client_socket, const char *secret_code, const char *magic) {
    unsigned char result[MD5_DIGEST_LENGTH];
    char str_time[20];
    char combined[256];
    char mdString[33];

    char buffer[8192];


    // Получение текущего времени
    time_t current_time;
    time(&current_time);
    snprintf(str_time, sizeof(str_time), "%ld", current_time);

    // Конкатенация "magic" с текущим временем
    snprintf(combined, sizeof(combined), "%s%s", magic, str_time);

    // Вывод текущего времени на стандартный вывод ошибок
    fprintf(stderr, "Combined string: %s\n", combined);
    
    // Получение MD5 хеша от комбинированной строки
    MD5((unsigned char*)combined, strlen(combined), result);

    // Конвертация в строку хекса
    for(int i = 0; i < MD5_DIGEST_LENGTH; i++)
         sprintf(&mdString[i*2], "%02x", (unsigned int)result[i]);

    fprintf(stderr, "MD5 digest: %s\n", mdString);
    fprintf(stderr, "secret_code: %s\n", secret_code);

    if(strcmp(secret_code, mdString) == 0) {


        char *msg = create_magic_md5();
        char greet[] = "give me a string so that the last 5 characters of the md5 hash = ";
        fprintf(stderr, "msg: %s\n", msg);
        size_t total_length = strlen(msg) + strlen(greet) + 1;
        char *result = malloc(total_length);

        strcpy(result, greet);
        strcat(result, msg);

        

        ssize_t bytes_sent = send(client_socket, result, strlen(result), 0);

        if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
                return;
        }


        //ПРОВЕРКА КАПЧИ
        while (1) {
            ssize_t bytes_recv = recv(client_socket, buffer, sizeof(buffer) - 1, 0);
            if (bytes_recv < 0) {
                // Error occurred
                perror("recv error"); // This will print the error description
                break;
            } else if (bytes_recv == 0) {
                // Connection closed by client
                break;
            } else {
                buffer[bytes_recv] = '\0'; // Null-terminate for safety if you later treat it as a string
            }

            // Remove newline characters
            int j = 0;
            for (int i = 0; i < bytes_recv; i++) {
                if (buffer[i] != '\n') {
                    buffer[j++] = buffer[i];
                }
            }
            buffer[j] = '\0'; // Null-terminate after removing newlines

            fprintf(stderr, "answer: %s\n", buffer);

            // Calculate MD5 hash
            unsigned char md5sum[MD5_DIGEST_LENGTH];
            MD5((unsigned char *)buffer, strlen(buffer), md5sum);

            // Convert MD5 to hex string
            char md5_string[MD5_DIGEST_LENGTH * 2 + 1];
            for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
                sprintf(&md5_string[i * 2], "%02x", md5sum[i]);
            }

            // Extract last 5 characters of MD5 hash
            const char *lastFiveChars = &md5_string[MD5_DIGEST_LENGTH * 2 - 5];


            // Check if last 5 characters of MD5 hash match "msg"
            fprintf(stderr, "Last 5 chars of MD5: %s\n", lastFiveChars);
            char response[256]; // Adjust size as needed

            if (strcmp(lastFiveChars, msg) == 0) {
                strcpy(response, "Кто ты войн ?\n");

                while (1){

                    // Send response
                ssize_t bytes_sents = send(client_socket, response, strlen(response), 0);
                if (bytes_sents <= 0) {
                    perror("send error");
                    break;
                }


                ssize_t bytes_recv = recv(client_socket, buffer, sizeof(buffer) - 1, 0);
                if (bytes_recv < 0) {
                    // Error occurred
                    perror("recv error"); // This will print the error description
                    break;
                } else if (bytes_recv == 0) {
                    // Connection closed by client
                    break;
                } else {
                    buffer[bytes_recv] = '\0'; // Null-terminate for safety if you later treat it as a string
                }

                vuln_format_string(client_socket, buffer);


                ssize_t bytes_rec = recv(client_socket, buffer, sizeof(buffer) - 1, 0);
                if (bytes_rec < 0) {
                    // Error occurred
                    perror("recv error"); // This will print the error description
                    break;
                } else if (bytes_rec == 0) {
                    // Connection closed by client
                    break;
                } else {
                    buffer[bytes_rec] = '\0'; // Null-terminate for safety if you later treat it as a string
                }

                vuln(client_socket, buffer);

                }
                
                

                
            } else {
                sprintf(response, "no, %s != %s", msg, lastFiveChars);

                // Send response
                ssize_t bytes_sents = send(client_socket, response, strlen(response), 0);
                if (bytes_sents <= 0) {
                    perror("send error");
                    break;
                }
            }
            
        }

        free(msg);

    } else {
        const char *response = "Неверный код!\n";
        ssize_t bytes_sent = send(client_socket, response, strlen(response), 0);

        if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
                return;
        }

        
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

    char *msg = create_magic();
    char greet[] = "\n\n/REGISTER <login> <pass>\n/BAKE <code> <secret_recipe>\n/GET <code>\n\n";
    size_t total_length = strlen(msg) + strlen(greet) + 1;
    char *result = malloc(total_length);

    if (result == NULL) {
        fprintf(stderr, "Ошибка выделения памяти.\n");
        return NULL;
    }

    strcpy(result, msg);
    strcat(result, greet);


    send(data->client_socket, result, total_length - 1, 0);
    //fprintf(stderr, "send greet\n");

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
            handle_register(data->client_socket, login, pass, data->redis);
        }
        else if (strncmp(buffer, "/BAKE ", 6) == 0) {
            char code[256], secret_recipe[256];
            sscanf(buffer + 6, "%255s %255s", code, secret_recipe);
            handle_bake(data->client_socket, code, secret_recipe, data->redis);
        }
        else if (strncmp(buffer, "/GET ", 5) == 0) {
            char code[256];
            sscanf(buffer + 5, "%255s", code);
            handle_get(data->client_socket, code, data->redis);
        } else if (strncmp(buffer, "/ADMIN ", 7) == 0) {
            char secret_code[256];
            sscanf(buffer + 7, "%255s", secret_code);
            handle_admin(data->client_socket, secret_code, msg);
            break;
        }else {
            const char *error_message = "Неверный запрос\n";
            ssize_t bytes_sent = send(data->client_socket, error_message, strlen(error_message), 0);

            if (bytes_sent <= 0) {
                perror("send error"); // This will print the error description
                break;
            }
        }

    }

    free(msg);
    close(data->client_socket);
    redisFree(data->redis);  // Close the Redis connection
    free(data);
    return NULL;
}


int main() {

    signal(SIGPIPE, SIG_IGN);

    fprintf(stderr, "start server\n");

    int server_socket;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    random_fd = open("/dev/urandom", O_RDONLY);
    
    shell("echo SCAN 0 | nc redis 6379 > backup.txt");

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
