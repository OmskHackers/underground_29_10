#include "crypt.h"
#include <stdlib.h>
#include <string.h>

void encryptPart(
    unsigned char* data,
    unsigned int size,
    unsigned char key,
    const unsigned char step,
    unsigned int block,
    unsigned int limit)
{
    const unsigned char* end;
    unsigned char* temp = malloc(block);
    unsigned int increment = (block >> 1) + (block & 1);

    if (size < block >> 2)
        size = 0;
    else
        size -= (size % block < block >> 2) * size % block + size % 2;

    if (limit % block != 0)
        limit = limit + (block - (limit % block));

    end = data + (size < limit ? size : limit);

    while (data < end) {
        unsigned char* in;
        unsigned char* out = temp;
        if (end - data < (long)block) {
            block = end - data;
            increment = (block >> 1) + (block & 1);
        }

        for (in = data + block - 1; in > data;) {
            *out = *in-- ^ key;
            *(out + increment) = *in-- ^ (key + step * increment);
            ++out;
            key += step;
        }

        if (block & 1) {
            *out = *in ^ key;
            key += step;
        }
        key += step * increment;

        memcpy(data, temp, block);
        data += block;
    }

    free(temp);
}

void decryptPart(
    unsigned char* data,
    unsigned int size,
    unsigned char key,
    const unsigned char step,
    unsigned int block,
    unsigned int limit)
{
    const unsigned char* end;
    unsigned char* temp = malloc(block);
    unsigned int increment = (block >> 1) + (block & 1);

    if (size < block >> 2)
        size = 0;
    else
        size -= (size % block < block >> 2) * size % block + size % 2;

    if (limit % block != 0)
        limit = limit + (block - (limit % block));

    end = data + (size < limit ? size : limit);

    while (data < end) {
        unsigned char* in = data;
        unsigned char* out;
        if (end - data < (long)block) {
            block = end - data;
            increment = (block >> 1) + (block & 1);
        }

        for (out = temp + block - 1; out > temp;) {
            *out-- = *in ^ key;
            *out-- = *(in + increment) ^ (key + step * increment);
            ++in;
            key += step;
        }

        if (block & 1) {
            *out = *in ^ key;
            key += step;
        }
        key += step * increment;

        memcpy(data, temp, block);
        data += block;
    }

    free(temp);
}
