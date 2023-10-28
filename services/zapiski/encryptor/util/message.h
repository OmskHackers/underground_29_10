#pragma once

#include <stddef.h>
#include <inttypes.h>
#include <errno.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef errno_t
#define errno_t int
#endif

typedef struct {
    uint32_t extra;
    // -1
    long int size;
    long int zsize;
    long int offset;
} Entry;

typedef struct MessageController MessageController;

typedef struct Message Message;

typedef struct Message {
    unsigned int messageType;
    const MessageController* controller;
    char* stream;
    size_t entryCount;
    Entry* entries;
    unsigned long offset;
    int inited;
} Message;

typedef struct MessageController {
    unsigned long flags;

    int (*open)(Message* message, errno_t** error);
    int (*create)(Message* message, errno_t** error);
    int (*close)(Message* message, errno_t** error);

    long int(*read)(Message* message, int entry, char* output, errno_t** error);
    long int(*write)(Message* message, int entry, char* input, size_t length, errno_t** error);
};

#ifdef __cplusplus
}
#endif