#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void initRand();

int randLength(const int min, const int max);

typedef struct RecollectElem RecollectElem;

typedef struct RecollectElem {
    char* buffer;
    const long int length;
} RecollectElem;

char generateChar();

int generateString(char* buffer, const int minLength, const int maxLength);

int compressString(char* buffer, const int length);

int recollect(char* buffer, const long int length, RecollectElem* elements, const long int elementsCount);

char* testSal();

#ifdef __cplusplus
}
#endif