#include "util.h"

#if IS_DAY(4)
void solve() {
	int32_t pt1 = 0;
	int32_t pt2 = 0;

	String s;
	for (;;) {
		if (!read_until(&s, '-')) {
			break;
		}
		uint16_t ll = s.toInt();
		read_until(&s, ',');
		uint16_t lr = s.toInt();
		read_until(&s, '-');
		uint16_t rl = s.toInt();
		read_until(&s, '\n');
		uint16_t rr = s.toInt();

		if (ll <= rr ? lr >= rl : rr >= ll) {
			pt2++;
			if (ll <= rl && lr >= rr || rl <= ll && rr >= lr) {
				pt1++;
			}
		}
	}

	Serial.print("\npt1: ");
	Serial.println(pt1);

	Serial.print("pt2: ");
	Serial.println(pt2);
}
#endif