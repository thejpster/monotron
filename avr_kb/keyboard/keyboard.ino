/**
 * Monotron I/O Controller.
 *
 * Copyright (c) Jonathan 'theJPster' Pallant 2019
 *
 * Licensed under the MIT or Apache 2.0 licences, at your option.
 *
 * Note that Arduino libraries are licensed under the LGPL, and so anyone
 * in receipt of a binary version of this firmware must also receive this
 * file as either source code or a .o file suitable for re-linking.
 */

// The Keyboard Clock pin (PC0)
const int KB_CLK_PIN = A0;
// The Mouse Clock pin (PC1)
const int MS_CLK_PIN = A1;
// The Keyboard Data pin (PC2)
const int KB_DATA_PIN = A2;
// The Mouse Data pin (PC2)
const int MS_DATA_PIN = A3;

const uint8_t PS2_NUM_BITS_IN_WORD = 11;

// The commands we handle
const uint8_t READ_KB_REQ = 0x30;
const uint8_t READ_KB_CFM_OK = 0x31;
const uint8_t READ_KB_CFM_ERR = 0x32;
const uint8_t DATA_KB_IND = 0x33;
const uint8_t READ_MS_REQ = 0x34;
const uint8_t READ_MS_CFM_OK = 0x35;
const uint8_t READ_MS_CFM_ERR = 0x36;
const uint8_t DATA_MS_IND = 0x37;
const uint8_t BAD_COMMAND_CFM = 0x38;

class Ps2Buffer {
public:
	bool want_sample() {
		return (this->num_bits < PS2_NUM_BITS_IN_WORD);
	}

	bool have_data() {
		return (this->num_bits == PS2_NUM_BITS_IN_WORD);
	}

	bool add_sample(bool sample) {
		if (sample) {
			this->word |= (1 << this->num_bits);
		}
		this->num_bits++;
		return (this->num_bits == PS2_NUM_BITS_IN_WORD);
	}

	uint8_t get_word() {
		uint8_t result = (this->word >> 1) & 0xFF;
		this->reset();
		return result;
	}

	void reset() {
		this->word = 0;
		this->num_bits = 0;
	}

private:
	uint16_t word;
	uint8_t num_bits;

};

static void ms_off(void) {
	pinMode(MS_CLK_PIN, OUTPUT);
	digitalWrite(MS_CLK_PIN, LOW);
}

static void ms_on(void) {
	pinMode(MS_CLK_PIN, INPUT_PULLUP);

}

static void kb_off(void) {
	pinMode(KB_CLK_PIN, OUTPUT);
	digitalWrite(KB_CLK_PIN, LOW);
}

static void kb_on(void) {
	pinMode(KB_CLK_PIN, INPUT_PULLUP);
}

/**
 * Executed once on startup.
 */
void setup(void) {
	// Configure the pins
	pinMode(KB_CLK_PIN, INPUT_PULLUP);
	pinMode(KB_DATA_PIN, INPUT_PULLUP);
	pinMode(MS_CLK_PIN, INPUT_PULLUP);
	pinMode(MS_DATA_PIN, INPUT_PULLUP);
	// Communication with the MCU. This gives 19,184 bps (0.08% error) at 8 MHz
	// or 19,231 bps (0.16% error) at 1 MHz
	Serial.begin(19200);
}

/**
 * Main execution loop.
 *
 * Checks for falling edges on the clock pins. When a falling edge is seen,
 * data bits are clocked in. When enough data bits have been seen PS/2 communication
 * is prevented (by holding CLK low) while we send the received data over the UART.
 */
void loop(void) {
	static Ps2Buffer kb;
	static Ps2Buffer ms;
	static bool old_kb_clk_pin = true;
	static bool old_ms_clk_pin = true;

	bool kb_clk_pin = digitalRead(KB_CLK_PIN);
	// capture data on falling edge
	if (old_kb_clk_pin && !kb_clk_pin) {
		if (kb.add_sample(digitalRead(KB_DATA_PIN))) {
			// Shut the keyboard down
			kb_off();
			// Send indication
			Serial.write(DATA_KB_IND);
		}
	}
	old_kb_clk_pin = kb_clk_pin;

	bool ms_clk_pin = digitalRead(MS_CLK_PIN);
	// capture data on falling edge
	if (old_ms_clk_pin && !ms_clk_pin) {
		if (ms.add_sample(digitalRead(MS_DATA_PIN))) {
			// Shut the mouse down
			ms_off();
			// Send indication
			Serial.write(DATA_MS_IND);
		}
	}
	old_ms_clk_pin = ms_clk_pin;

	if (Serial.available()) {
		// Shut down devices while we deal with this
		kb_off();
		ms_off();
		uint8_t command = Serial.read();
		switch (command) {
		case READ_KB_REQ:
			if (kb.have_data()) {
				Serial.write(READ_KB_CFM_OK);
				Serial.write(kb.get_word());
				kb.reset();
			} else {
				Serial.write(READ_KB_CFM_ERR);
			}
			break;
		case READ_MS_REQ:
			if (ms.have_data()) {
				Serial.write(READ_MS_CFM_OK);
				Serial.write(ms.get_word());
				ms.reset();
			} else {
				Serial.write(READ_MS_CFM_ERR);
			}
			break;
		default:
			Serial.write(BAD_COMMAND_CFM);
			break;
		}
		// Re-enable devices
		if (!kb.have_data()) {
			// Only re-enable if it is ready for more data
			kb.reset();
			kb_on();
			old_kb_clk_pin = false;
		}
		if (!ms.have_data()) {
			// Only re-enable if it is ready for more data
			ms.reset();
			ms_on();
			old_ms_clk_pin = false;
		}
	}
}
