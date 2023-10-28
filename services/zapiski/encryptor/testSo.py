import ctypes

libSo = ctypes.CDLL('./encryptor.so')
libSo2 = ctypes.cdll.LoadLibrary('./encryptor.so')

# libSo2.init()
# libSo.init()

# res = libSo.testRand()
# res2 = libSo.testRand()

# print(res)
# print(res2)

testLibSo = ctypes.CDLL('./testingSth.so')

testEndianness = testLibSo.testEndianness
buf = ctypes.create_string_buffer(4)
testEndianness(buf)
print(buf.value) # yields b'abcd'
print("big-endian = " + str(ord('a') + ord('b') * 256 + ord('c') * 256 * 256 + ord('d') * 256 * 256 * 256))
print(int.from_bytes(buf.value, byteorder='little')) # meaning need to convert int to little

testEndianness2 = testLibSo.testEndianness2
testEndianness2(buf)
print(buf.value) # yields b'd\0\0\0'
print("big-endian = " + str(ord('d'))) # hz
print(int.from_bytes(buf.value, byteorder='little')) # meaning need to convert int to little


# res = libSo2.testRand()
# res2 = libSo2.testRand()

# print(res)
# print(res2)

# charStr = b'1234'
# print(charStr)
# charArr = bytearray(charStr)
# charArr[0] = ord('7')
# print(bytes(charArr))
# print(charStr)