#include "util.h"

#if IS_DAY(5)

struct Stack {
	uint8_t len;
	uint8_t cap;
	uint8_t data[0];
};

Stack *new_stack(size_t len) {
	Stack *s = (Stack*) malloc(len + sizeof(Stack));
	if (!s) {
		Serial.println("Fuck");
		Serial.flush();
	}
	s->len = 0;
	s->cap = len;
	return s;
}

void stack_push(Stack *&stack, uint8_t value) {
	size_t o = stack->len;
	if (++stack->len > stack->cap) {
		size_t new_cap = stack->cap + 8;
		stack = (Stack*) realloc(stack, sizeof(stack) + new_cap);
		stack->cap = new_cap;
	}
	stack->data[o] = value;
}

void solve() {
	Stack *pt1[9];

	for (size_t i = 0; i < ARRAY_LENGTH(pt1); i++) {
		pt1[i] = new_stack(8);
	}

	String s;
	for (;;) {
		if (!read_until(&s, '\n', 38)) {
			break;
		}
		if (s.length() <= 0) {
			break;
		}

		for (size_t i = 1; i < s.length(); i += 4) {
			uint8_t v = s[i];
			if (v != ' ' && !(v >= '0' && v <= '9')) {
				stack_push(pt1[i / 4], v);
			}
		}
	}

	// we read in the stacks upside down, so flip them & dup them into pt2
	Stack *pt2[ARRAY_LENGTH(pt1)];
	for (size_t i = 0; i < ARRAY_LENGTH(pt1); i++) {
		pt2[i] = new_stack(pt1[i]->cap);
		for (intptr_t a = 0, b = pt1[i]->len - 1; b >= 0; a++, b--) {
			pt2[i]->data[b] = pt1[i]->data[a];
		}
		pt2[i]->len = pt1[i]->len;
		memcpy(pt1[i]->data, pt2[i]->data, pt1[i]->len);
	}

	for (;;) {
		if (!read_until(&s, ' ')) {
			break;
		}
		read_until(&s, ' ');
		uint8_t count = s.toInt();
		read_until(&s, ' ');
		read_until(&s, ' ');
		uint8_t from = s.toInt() - 1;
		read_until(&s, ' ');
		read_until(&s, '\n');
		uint8_t to = s.toInt() - 1;

		{
			Stack *f = pt1[from];
			for (uint8_t i = 0; i < count; i++) {
				stack_push(pt1[to], f->data[--f->len]);
			}
		}
		{
			Stack *f = pt2[from];
			for (uint8_t end = f->len, p = f->len -= count; p < end; p++) {
				stack_push(pt2[to], f->data[p]);
			}
		}
	}

	Serial.print("\npt1: ");
	for (size_t i = 0; i < ARRAY_LENGTH(pt1); i++) {
		Stack *s = pt1[i];
		if (s->len) {
			Serial.write(&s->data[s->len - 1], 1);
		}
	}
	Serial.println("");
	Serial.print("pt2: ");
	for (size_t i = 0; i < ARRAY_LENGTH(pt2); i++) {
		Stack *s = pt2[i];
		if (s->len) {
			Serial.write(&s->data[s->len - 1], 1);
		}
	}
	Serial.println("");
}
#endif