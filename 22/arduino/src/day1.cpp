#include "util.h"

#if IS_DAY(1)
void solve() {
	int32_t max[3] = {0};

	int32_t v = 0;
	for (String line; read_until(&line) || v != 0; ) {
		if (line.length() <= 0) {
			if (max[0] < v) {
				max[0] = v;
				for (int i = 1; i < ARRAY_LENGTH(max); i++) {
					if (max[i - 1] > max[i]) {
						int32_t v = max[i - 1];
						max[i - 1] = max[i];
						max[i] = v;
					} else {
						break;
					}
				}
			}
			v = 0;
		} else {
			v += line.toInt();
		}
	}

	Serial.print("\npt1: ");
	Serial.println(max[2]);

	int32_t sum = 0;
	for (size_t i = 0; i < ARRAY_LENGTH(max); i++) {
		sum += max[i];
	}
	Serial.print("pt2: ");
	Serial.println(sum);
}
#endif