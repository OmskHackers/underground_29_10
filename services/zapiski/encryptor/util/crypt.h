#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void encryptPart(
    unsigned char* data,
    unsigned int size,
    unsigned char key,
    const unsigned char step,
    unsigned int block,
    unsigned int limit
);

void decryptPart(
    unsigned char* data,
    unsigned int size,
    unsigned char key,
    const unsigned char step,
    unsigned int block,
    unsigned int limit
);

#ifdef __cplusplus
}
#endif