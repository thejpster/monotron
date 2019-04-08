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
const int KB_CLK = A0;
// The Mouse Clock pin (PC1)
const int MS_CLK = A1;
// The Keyboard Data pin (PC2)
const int KB_DATA = A2;
// The Mouse Data pin (PC2)
const int MS_DATA = A3;

/**
 * Executed once on startup.
 */
void setup(void) {
  // Configure the pins
  pinMode(KB_CLK, INPUT_PULLUP);
  pinMode(KB_DATA, INPUT_PULLUP);
  pinMode(MS_CLK, INPUT_PULLUP);
  pinMode(MS_DATA, INPUT_PULLUP);
  // Communication with the MCU
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
  static bool old_kb_clk_pin = true;
  static uint16_t kb_word = 0;
  static uint8_t kb_num_bits = 0;

  bool kb_clk_pin = digitalRead(KB_CLK);
  // capture data on falling edge
  if (old_kb_clk_pin && !kb_clk_pin) {
    if (digitalRead(KB_DATA)) {
      kb_word |= (1 << kb_num_bits);
    }
    kb_num_bits++;
    if (kb_num_bits == 11) {
      // Shut the keyboard and mouse down for a moment
      pinMode(KB_CLK, OUTPUT);
      digitalWrite(KB_CLK, LOW);
      // Print the word we just read. We shift by one to trim the start bit (0)
      // and mask with 0xFF to trim the stop and parity bits.
      Serial.print((char) ((kb_word >> 1) & 0xFF));
      // Clear the word we just read
      kb_word = 0;
      kb_num_bits = 0;
      // Release the keyboard
      pinMode(KB_CLK, INPUT_PULLUP);
    }
  }
  old_kb_clk_pin = kb_clk_pin;
}
