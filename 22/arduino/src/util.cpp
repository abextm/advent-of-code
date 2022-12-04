#include "util.h"


void setup() __attribute__((weak));
void setup() { }

void loop()
{
	for (; !Serial; );
	
	solve();

	Serial.print('\0');
	Serial.flush();
	for (; Serial; );
}


int16_t line_no = 1;
bool hit_eol = false;
bool read_until(String *into, char delimiter, size_t max_len) {
	*into = "";
	if (hit_eol) {
		return false;
	}
	for (; into->length() < max_len; ) {
		char c = Serial.read();
		if (c == -1) {
			continue;
		}
		if (c == '\n') {
			line_no++;
		}
		if (c == delimiter) {
			return true;
		}
		if (c == '\0') {
			hit_eol = true;
			return false;
		}
		if (!into->concat(c)) {
			Serial.print(line_no);
			Serial.println(F(": read_until oom"));
			return true;
		}
	}

	Serial.print(line_no);
	Serial.println(F(": read_until hit limit"));
	return true;
}