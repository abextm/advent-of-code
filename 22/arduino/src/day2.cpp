#include "util.h"

#if IS_DAY(2)
void solve() {
	int32_t pt1 = 0;
	int32_t pt2 = 0;
	for (String line; read_until(&line); ) {
		uint8_t opponent = line.charAt(0) - 'A';
		uint8_t outcome_self = line.charAt(2) - 'X';

		{
			uint8_t win = 3 * ((outcome_self + 4 - opponent) % 3);
			uint8_t score = outcome_self + 1;
			pt1 += win + score;
		}
		{
			uint8_t win = outcome_self * 3;
			uint8_t me= (outcome_self + 2 + opponent) % 3;
			pt2 += win + me + 1;
		}
	}

	Serial.print("\npt1: ");
	Serial.println(pt1);

	Serial.print("pt2: ");
	Serial.println(pt2);
}
#endif