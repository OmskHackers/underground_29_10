import ctypes
import util

testLibSo = ctypes.CDLL('./testingSth.so')

testRecollect = testLibSo.testRecollect
testRecollect.argtypes = [ctypes.c_char_p, ctypes.c_char_p] # text, out
text = "input string from python".encode('utf-8')
out = ctypes.create_string_buffer(1024)
error = testRecollect(text, out)
if error == 0:
    print('success!')
    print(out.value)
    print(util.parseRecollected(out.raw))
else:
    print('an error has occured')