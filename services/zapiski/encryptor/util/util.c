#include "util.h"
#include <stdlib.h>
#include <time.h>
#include <stdbool.h>
#include <string.h>

#include <stdio.h>

static const char MIN_CHAR = 32;
static const char MAX_CHAR = 127;
static const char DIF_CHAR = 94; // 127 - 32 - 1 

static bool isRandInit = false;

void initRand() {
    #pragma omp single
    {
        if (!isRandInit) {
            srand(time(NULL));
            isRandInit = true;
            printf("Rand initiated\n");
        }
    }
}

int randLength(const int min, const int max) {
    int randValue;
    // this is an ok bottleneck
    #pragma omp critical 
    {
        randValue = rand();
    }
    return min + randValue % (max - min);
}

char generateChar(unsigned int unrand) {
    return MIN_CHAR + (unrand % DIF_CHAR);
}

int generateString(char* buffer, const int minLength, const int maxLength) {
    int length = randLength(minLength, maxLength);
    unsigned int unrand = (unsigned int)randLength(1, DIF_CHAR);
    for (int i = 0; i < length - 1; i++) {
        buffer[i] = generateChar(unrand);
        unrand *= 2;
    }
    buffer[length - 1] = '\0';
    return length;
}

int compressString(char* buffer, const int length) {
    if (length <= 0)
        return length;

    char curSymbol = buffer[0];
    int curSymbolCount = 1;
    int index = 0;
    for (int i = 1; i < length; i++) {
        if (curSymbol == buffer[i]) {
            if (curSymbolCount == 31) {
                buffer[index++] = curSymbolCount;
                buffer[index++] = curSymbol;
                curSymbolCount = 0;
            }
            curSymbolCount++;
        }
        else {
            if (curSymbolCount > 1)
                buffer[index++] = curSymbolCount;
            buffer[index++] = curSymbol;
            curSymbol = buffer[i];
            curSymbolCount = 1;
        }
    }
    buffer[index++] = '\0';
    return index; // length of compressed string
}

int recollect(char* buffer, const long int length, RecollectElem* elements, const long int elementsCount) {
    int idx = 0;
    if (idx + sizeof(elementsCount) > length) {
        printf("%d + %ld > %ld\n", idx, sizeof(elementsCount), length);
        return -1;
    }
    memcpy(buffer + idx, (const char*)&elementsCount, sizeof(elementsCount));
    idx += sizeof(elementsCount);
    for (int i = 0; i < elementsCount; i++) {
        if (idx + sizeof(elements[i].length) > length) {
            printf("%d + %ld > %ld\n", idx, sizeof(elements[i].length), length);
            return -1;
        }
        memcpy(buffer + idx, (const char*)&elements[i].length, sizeof(elements[i].length));
        idx += sizeof(elements[i].length);
        if (idx + elements[i].length > length) {
            printf("%d + %ld > %ld\n", idx, elements[i].length, length);
            return -1;
        }
        memcpy(buffer + idx, elements[i].buffer, elements[i].length);
        idx += elements[i].length;
    }
    return 0;
}

char* testSal() {
    return "salo rabotaet";
}
