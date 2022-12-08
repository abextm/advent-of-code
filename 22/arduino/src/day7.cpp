#include "util.h"

#if IS_DAY(7)

typedef uint8_t DirPtr;
#define DP_NULL 0xFF

struct Dir;

Dir *root;
uint8_t bump_ptr;

struct Dir {
	uint8_t parent;
	uint8_t sibling;
	uint8_t child;
	uint32_t size;
	uint16_t hash;
};

void dir_add_size(DirPtr d, uint32_t count) {
	for (; d != DP_NULL; d = root[d].parent) {
		root[d].size += count;
	}
}

DirPtr dir_get_subdir(DirPtr self, uint16_t hash) {
	for (DirPtr d = root[self].child; d != DP_NULL; d = root[d].sibling) {
		if (root[d].hash == hash) {
			return d;
		}
	}

	DirPtr n = bump_ptr++;
	if (n == DP_NULL) {
		Serial.print(F("OOM"));
	}

	root[n].parent = self;
	root[n].sibling = root[self].child;
	root[n].child = DP_NULL;
	root[n].size = 0;
	root[n].hash = hash;
	root[self].child = n;

	return n;
}

char *split(char *str, char delim) {
	for (;; str++) {
		if (*str == '\0') {
			return NULL;
		}
		if (*str == delim) {
			*str = '\0';
			return str + 1;
		}
	}
}

uint16_t hash(const char *str) {
	uint16_t h = 0;
	for (; *str; str++) {
		h = (h * 0b1101) + *str;
	}
	return h;
}


uint32_t pt1;
uint32_t pt2;
void find_values(DirPtr d, uint32_t min_free) {
	for (; d != DP_NULL; d = root[d].sibling) {
		if (root[d].size <= 100000) {
			pt1 +=root[d].size;
		}
		if (root[d].size >= min_free && root[d].size < pt2) {
			pt2 = root[d].size;
		}
		find_values(root[d].child, min_free);
	}
}


extern bool hit_eol;

extern char __heap_start;
extern char *__brkval;

void print_mem_stats() {
	Serial.print(F("heap start "));
	Serial.println((size_t) &__heap_start);
	Serial.print(F("heap break "));
	Serial.print((size_t) __brkval);
	Serial.print(F(", used "));
	size_t used = 0;
	if (__brkval != 0) {
		used = (size_t) __brkval - (size_t) &__heap_start;
	}
	Serial.println(used);
	
	Serial.print(F("stack ptr "));
	Serial.print((size_t) SP);
	Serial.print(F(", used "));
	Serial.println(0xAFF - (size_t) SP);
}

void solve() {
	String2 s(32);
	read_until(&s);

	root = (Dir*) malloc(216 * sizeof(Dir));
	root->parent = DP_NULL;
	root->sibling = DP_NULL;
	root->child= DP_NULL;
	root->size = 0;
	root->hash = hash("/");
	print_mem_stats();
	bump_ptr = 1;

	uint32_t total = 0;

	for (DirPtr dir = 0; !hit_eol;) {
		for (;;) {
			char *str = const_cast<char*>(s.c_str());

			str = split(str, ' '); //skip $
			char *arg = split(str, ' ');
			if (*str == 'c') {
				if (*arg == '/') {
					dir = 0;
				} else if (*arg ==  '.') {
					dir = root[dir].parent;
				} else {
					dir = dir_get_subdir(dir, hash(arg));
				}
			} else if (*str == 'l') {
				break;
			} else {
				Serial.print(F("bad sh "));
				Serial.println(str);
			}

			if (!read_until(&s)) {
				break;
			}
		}

		uint32_t size = 0;
		for (; read_until(&s) && !s.startsWith("$"); ) {
			char *str = const_cast<char*>(s.c_str());
			char *name = split(str, ' ');
			if (*str == 'd') {
				continue;
			}
			size_t len = name - str;
			s.remove(len, s.length() - len);
			size += s.toInt();
		}

		total += size;
		dir_add_size(dir, size);
	}

	uint32_t min_free = root->size - (70000000 - 30000000);
	pt1 = 0;
	pt2 = 0xFFFFFFFF;
	find_values(0, min_free);

	free(root);

	Serial.print(F("pt1: "));
	Serial.println(pt1);
	Serial.print(F("pt2: "));
	Serial.println(pt2);
}

#endif