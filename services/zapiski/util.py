BYTES_IN_LONG_INT = 8 # cuz 64-bit system

def parseRecollected(recollectedChars : bytes) -> list[bytes]:
    idx = 0
    elementsCount = int.from_bytes(recollectedChars[idx:idx + BYTES_IN_LONG_INT], byteorder='little')
    idx += BYTES_IN_LONG_INT
    parsedStrings = []
    for i in range(elementsCount):
        stringLength = int.from_bytes(recollectedChars[idx:idx + BYTES_IN_LONG_INT], byteorder='little')
        idx += BYTES_IN_LONG_INT
        string = recollectedChars[idx:idx + stringLength]
        idx += stringLength
        parsedStrings.append(string)
    return parsedStrings

