/**
   Monotron I/O Controller.

   Copyright (c) Jonathan 'theJPster' Pallant 2019

   Licensed under the MIT or Apache 2.0 licences, at your option.

   Note that Arduino libraries are licensed under the LGPL, and so anyone
   in receipt of a binary version of this firmware must also receive this
   file as either source code or a .o file suitable for re-linking.
*/

// The Keyboard Clock pin (PC0)
const int KB_CLK = A0;
// The Mouse Clock pin (PC1)
const int MS_CLK = A1;
// The Keyboard Data pin (PC2)
const int KB_DATA = A2;
// The Mouse Data pin (PC3)
const int MS_DATA = A3;
// The Parallel Port Init pin (PC4)
const int LPT_INIT = A4;
// The Parallel Port Select pin (PC5)
const int LPT_SELECT = A5;
// Not supported as PC6 is the !RESET pin.
// The Parallel Port Select Printer / Select In pin (PC6)
// const int LPT_SELIN = A6;
// The Parallel Port Ack pin (PD2)
const int LPT_ACK = 2;
// The Parallel Port Busy pin (PD3)
const int LPT_BUSY = 3;
// The Parallel Port Paper End pin (PD4)
const int LPT_PE = 4;
// The Parallel Port Error pin (PD5)
const int LPT_ERROR = 5;
// The Parallel Port Auto Feed pin (PD6)
const int LPT_AUTOF = 6;
// The Parallel Port Strobe pin (PD7)
const int LPT_STROBE = 7;

const uint16_t PRINTER_TIMEOUT = 20000;

const size_t BUFFER_LEN = 32;

class Ps2Receiver {
  public:
    Ps2Receiver(int clk, int dat);
    void poll();
    bool has_data();
    uint8_t get_data();
    void disable();
    void enable();
  private:
    // Buffers up bytes from this device
    int clk_pin;
    int dat_pin;
    uint16_t read_word;
    uint8_t num_bits;
    bool old_clk_pin_value;
    char buffer[BUFFER_LEN];
    size_t read;
    size_t written;
    size_t used;
};

Ps2Receiver::Ps2Receiver(int clk, int dat):
  clk_pin(clk),
  dat_pin(dat),
  read_word(0),
  num_bits(0),
  old_clk_pin_value(false),
  written(0),
  read(0),
  used(0)
{
  pinMode(this->clk_pin, INPUT_PULLUP);
  pinMode(this->dat_pin, INPUT_PULLUP);
}

void Ps2Receiver::disable() {
  pinMode(this->clk_pin, OUTPUT);
  digitalWrite(this->clk_pin, LOW);
  this->num_bits = 0;
  this->read_word = 0;
  this->old_clk_pin_value = false;
}

void Ps2Receiver::enable() {
  pinMode(this->clk_pin, INPUT_PULLUP);
}

void Ps2Receiver::poll() {
  bool new_clk_pin_value = digitalRead(this->clk_pin);
  // capture data on falling edge
  if (this->old_clk_pin_value && !new_clk_pin_value) {
    if (digitalRead(this->dat_pin)) {
      this->read_word |= (1 << this->num_bits);
    }
    this->num_bits++;
    if (this->num_bits == 11) {
      uint8_t data = (this->read_word >> 1) & 0xFF;
      this->buffer[this->written % BUFFER_LEN] = data;
      this->written++;
      this->used++;
      if (this->used == BUFFER_LEN) {
        // Buffer full!
        this->disable();
      }
      // Clear the word we just read
      this->read_word = 0;
      this->num_bits = 0;
    }
  }
  this->old_clk_pin_value = new_clk_pin_value;
}

bool Ps2Receiver::has_data() {
  return (this->used > 0);
}

uint8_t Ps2Receiver::get_data() {
  uint8_t data = this->buffer[this->read % BUFFER_LEN];
  this->read++;
  this->used--;
  this->enable();
  return data;
}

static Ps2Receiver keyboard = Ps2Receiver(KB_CLK, KB_DATA);
static Ps2Receiver mouse = Ps2Receiver(MS_CLK, MS_DATA);

/**
   Executed once on startup.
*/
void setup(void) {
  // Communication with the MCU
  Serial.begin(19200);
  pinMode(LPT_INIT, OUTPUT);
  digitalWrite(LPT_INIT, LOW);
  pinMode(LPT_AUTOF, OUTPUT);
  digitalWrite(LPT_AUTOF, LOW);
  pinMode(LPT_STROBE, OUTPUT);
  digitalWrite(LPT_STROBE, LOW);
  // We don't support LPT_SELIN
  // pinMode(LPT_SELIN, OUTPUT);
  pinMode(LPT_SELECT, INPUT_PULLUP);
  pinMode(LPT_ACK, INPUT_PULLUP);
  pinMode(LPT_BUSY, INPUT_PULLUP);
  pinMode(LPT_PE, INPUT_PULLUP);
  pinMode(LPT_ERROR, INPUT_PULLUP);
  // Set all of PORT B to output
  DDRB = 0xFF;
}

/**
   Main execution loop.

   Checks for falling edges on the clock pins. When a falling edge is seen,
   data bits are clocked in. When enough data bits have been seen PS/2 communication
   is prevented (by holding CLK low) while we send the received data over the UART.
*/
void loop(void) {
  static bool is_printer_data = false;
  static bool is_printer_ctrl = false;
  keyboard.poll();
  mouse.poll();
  if (Serial.available()) {
    uint8_t command = Serial.read();
    if (is_printer_data) {
      mouse.disable();
      keyboard.disable();
      // Output data
      PORTB = command;
      for (uint16_t count = 0; count < PRINTER_TIMEOUT; count++) {
        // Spin until printer not busy
        if (digitalRead(LPT_BUSY) == false) {
          break;
        }
      }
      digitalWrite(LPT_STROBE, HIGH);
      for (uint16_t count = 0; count < PRINTER_TIMEOUT; count++) {
        // Spin until printer busy
        if (digitalRead(LPT_BUSY) == true) {
          break;
        }
      }
      digitalWrite(LPT_STROBE, LOW);
      is_printer_data = false;
      mouse.enable();
      keyboard.enable();
    } else if (is_printer_ctrl) {
      digitalWrite(LPT_INIT, (command & 1) != 0);
      digitalWrite(LPT_AUTOF, (command & 2) != 0);
      digitalWrite(LPT_STROBE, (command & 4) != 0);
      is_printer_ctrl = false;
    } else {
      switch (command) {
        case 'K':
          if (keyboard.has_data()) {
            uint8_t byte = keyboard.get_data();
            Serial.write(byte);
          } else {
            Serial.write('\0');
          }
          break;
        case 'M':
          if (mouse.has_data()) {
            uint8_t byte = mouse.get_data();
            Serial.write(byte);
          } else {
            Serial.write('\0');
          }
          break;
        case 'D':
          // Printer data byte comes next
          is_printer_data = true;
          break;
        case 'C':
          // Printer control byte comes next
          is_printer_ctrl = true;
          break;
        case 'R':
          {
            // Read printer stats
            uint8_t data = 0;
            data |= digitalRead(LPT_SELECT) << 0;
            data |= digitalRead(LPT_ACK) << 1;
            data |= digitalRead(LPT_PE) << 2;
            data |= digitalRead(LPT_BUSY) << 3;
            data |= digitalRead(LPT_ERROR) << 4;
            Serial.write(data);
          }
          break;
        default:
          Serial.write('\0');
          break;
      }

    }
  }
}
