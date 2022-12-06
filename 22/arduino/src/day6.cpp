#include "util.h"

#if IS_DAY(6)

#define BUF_SIZE 14

uint16_t find(char *buf, uint16_t *offset, uint8_t count) {
	uint16_t off = *offset;
	for (; off < count; off++) {
		buf[off] = read_blocking();
	}
	for (;;) {
		buf[off % BUF_SIZE] = read_blocking();
		for (uint8_t j = 0; j < count; j++) {
			for (uint8_t k = 0; k < count; k++) {
				if (j != k) {
					if (buf[(off - j) % BUF_SIZE] == buf[(off - k) % BUF_SIZE]) {
						goto next;
					}
				}
			}
		}

		return *offset = off + 1;
	next:
		off++;
	}
}

void solve() {
	char buf[BUF_SIZE];
	uint16_t offset = 0;
	uint16_t pt1 = find(buf, &offset, 4);
	uint16_t pt2 = find(buf, &offset, 14);

	for (; read_blocking(); );

	Serial.print("pt1: ");
	Serial.println(pt1);
	Serial.print("pt2: ");
	Serial.println(pt2);
}
#endif