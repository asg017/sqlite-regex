gcc -O3 -shared -fPIC regexp.c -o regexp.dylib

gcc -O3 -shared -fPIC -I./ re.c sqlite3-re.c -o re.dylib
