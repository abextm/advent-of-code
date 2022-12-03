#include "util.h"

#if IS_DAY(3)
int32_t pt1;
int32_t pt2;

int32_t score(char c) {
	uint8_t s = (c | ('a' ^ 'A')) - 'a';
	return s + (c >= 'a' ? 1 : 27);
}

void part1(String *s) {
	size_t len = s->length();
	size_t mid = len / 2;
	const char *cs = s->c_str();
	for (size_t a = 0; a < mid; a++) {
		for (size_t b = mid; b < len; b++) {
			if (cs[a] == cs[b]) {
				pt1 += score(cs[a]);
				return;
			}
		}
	}
	Serial.println("no match");
}

void solve() {
	pt1 = pt2 = 0;
	String as, bs;
	for (;;) {
		if (!read_until(&as, '\n', 64)) {
			break;
		}
		part1(&as);

		for (int i = 1; i < 3; i++) {
			read_until(&bs, '\n', 64);
			part1(&bs);

			size_t o = 0;
			for (size_t a = 0; a < as.length(); a++) {
				for (size_t b = 0; b < bs.length(); b++) {
					if (as.charAt(a) == bs.charAt(b)) {
						goto contains;
					}
				}
				continue;
			contains:
				as.setCharAt(o++, as.charAt(a));
			}
			as.remove(o, as.length() - o);
		}

		pt2 += score(as.charAt(0));
	}

	Serial.print("\npt1: ");
	Serial.println(pt1);

	Serial.print("pt2: ");
	Serial.println(pt2);
}
#endif