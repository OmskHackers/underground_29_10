#include "util/util.h"
#include "util/crypt.h"

#include <malloc.h>

#ifdef __cplusplus
extern "C" {
#endif

#define ENCRYPTED_ELEMENTS_COUNT 3

void init() {
    initRand();
}

int encrypt(const char* text, char* output) {
    // [0:64] err [64:128] key [66:2048] [2048] \0
    // 
    char error[64];
    char encrypted[256];
    char key[128];

    char precursor[8][32];
    int precursorCount = randLength(3, 8);
    for (int i = 0; i < precursorCount; i++) {
        int length = generateString(precursor[i], 3, 8);
        length = compressString(precursor[i], length);
    }

    char postcursor[8][32];
    int postcursorCount = randLength(3, 8);
    for (int i = 0; i < postcursorCount; i++) {
        generateString(postcursor[i], 3, 8);
    }

    RecollectElem elements[] = {
        { error, sizeof(error) },
        { encrypted, sizeof(encrypted) },
        { key, sizeof(key) }
    };
    
    if (recollect(output, sizeof(output), elements, 3) != 0) {
        // char recollectError[] = "error recollecting result";
        // char errorResult[4096];
        // RecollectElem errorElems[] = { {recollectError, sizeof(recollectError) } };
        // if (!recollect(errorResult, sizeof(errorResult), errorElems, 1))
        //     return "error recollecting error";
        // char* errorResultPtr = errorResult; // might(?) corrupt memory
        return -1;
    }

    // char* resultPtr = result; // might(?) corrupt memory
    return 0;
};

int testRand() {
    int result = randLength(1, 10);
    return result + randLength(1, 10);
}

#ifdef __cplusplus
}
#endif
