import ctypes
import base64

import util

BUFFER_SIZE = 1024

_libSo = None

_encrypt = None


def _getLib() -> ctypes.CDLL:
    global _libSo
    if not _libSo:
        _libSo = ctypes.CDLL('./encryptor/encryptor.so')
    return _libSo

def encrypt(message : str) -> tuple[str]:
    global _encrypt
    if not _encrypt:
        _encrypt = _getLib().encrypt
        _encrypt.argtypes = [ctypes.c_char_p, ctypes.c_char_p] # text, output
    messageBytes = message.encode('ascii')
    outCharPtr = ctypes.create_string_buffer(BUFFER_SIZE)
    _encrypt(messageBytes, outCharPtr)
    out = util.parseRecollected(outCharPtr)
    if len(out) < 3:
        return ('error parsing parsing recollection, len(out) < 3', '', '')
    error = out[0].decode('ascii')
    encryptedMessage = base64.b64encode(out[1]).decode('ascii')
    key = base64.b64encode(out[2]).decode('ascii')
    return (error, encryptedMessage, key)
