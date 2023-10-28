#include <memory.h>
#include "util/util.h"

#ifdef __cplusplus
extern "C" {
#endif

void testEndianness(char* output) {
    unsigned long int endian = 'a' + 
                      'b' * 256 +
                      'c' * 256 * 256 +
                      'd' * 256 * 256 * 256; 
    memcpy(output, (const char*)&endian, sizeof(endian));
    // return str; // returns b'abcd' meaning that it is big-endian (atleast in this architecture)
}

void testEndianness2(char* output) {
    unsigned long int endian = 'd';
    memcpy(output, (const char*)&endian, sizeof(endian));
}

int testRecollect(const char* text, char* output) {
    char error[64] = {};
    char inputed[256] = {};
    char key[128] = {};

    char errorText[] = "error is this";
    memcpy(error, errorText, sizeof(errorText));
    strcpy(inputed, text);
    char* keyText = "this is key";
    // memcpy(key, keyText, sizeof(keyText)); // this will not do
    memcpy(key, keyText, strlen(keyText)); // this will not do

    RecollectElem elements[] = {
        { error, strlen(error) },
        { inputed, strlen(inputed) },
        { key, strlen(key) }
    };
    
    if (recollect(output, 1024, elements, 3) != 0) {
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
}

#ifdef __cplusplus
}
#endif
