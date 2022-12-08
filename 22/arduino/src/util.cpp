#include "util.h"

String2::String2(size_t len) {
	this->buffer = (char*) calloc(1, len);
	this->len = 0;
	this->capacity = len;
}

void setup() __attribute__((weak));
void setup() { }

void loop()
{
	for (; !Serial; );
	
	unsigned long start = millis();
	solve();
	unsigned long duration = millis() - start;
	Serial.print(F("runtime: "));
	Serial.print(duration);
	Serial.print(F("ms"));

	Serial.print('\0');
	Serial.flush();
	for (; Serial; );
}

uint8_t read_blocking() {
	for (;;) {
		uint8_t c = Serial.read();
		if (c != -1) {
			return c;
		}
	}
}

int16_t line_no = 1;
bool hit_eol = false;
bool read_until(String *into, char delimiter, size_t max_len) {
	*into = "";
	if (hit_eol) {
		return false;
	}
	for (; into->length() < max_len; ) {
		char c = read_blocking();
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