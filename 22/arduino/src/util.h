#ifndef _UTIL_H
#define _UTIL_H

#include <Arduino.h>

#ifndef DAY
#	define IS_DAY(N) true
#else
#	define IS_DAY(N) DAY == N
#endif

void solve();
uint8_t read_blocking();
bool read_until(String *into, char delimiter = '\n', size_t max_len = 32);

#define ARRAY_LENGTH(x) (sizeof(x) / sizeof((x)[0]))

#endif // _UTIL_H