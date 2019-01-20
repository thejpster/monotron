/** @file main.c
 *
 * I/O Controller Firmware for Monotron. Implements an SPP LPT port, and a
 * PS/2 keyboard and mouse port.
 *
 * | Port   | Pin |  Direction     | Name          | Routed to  | Description              |
 * |--------|-----|----------------|------------- -|------------|--------------------------|
 * | B      | 0   | Output         | LPT_D0        | J13 pin 2  | LPT Output bit 0         |
 * | B      | 1   | Output         | LPT_D1        | J13 pin 3  | LPT Output bit 1         |
 * | B      | 2   | Output         | LPT_D2        | J13 pin 4  | LPT Output bit 2         |
 * | B      | 3   | Output         | LPT_D3        | J13 pin 5  | LPT Output bit 3         |
 * | B      | 4   | Output         | LPT_D4        | J13 pin 6  | LPT Output bit 4         |
 * | B      | 5   | Output         | LPT_D5        | J13 pin 7  | LPT Output bit 5         |
 * | B      | 6   | Output         | LPT_D6        | J13 pin 8  | LPT Output bit 6         |
 * | B      | 7   | Output         | LPT_D7        | J13 pin 9  | LPT Output bit 7         |
 * | C      | 0   | I/O Open-drain | KB_CLK        | J14 pin 5  | PS/2 Keyboard clock      |
 * | C      | 1   | I/O Open-drain | MS_CLK        | J15 pin 5  | PS/2 Mouse clock         |
 * | C      | 2   | I/O Open-drain | KB_DATA       | J14 pin 1  | PS/2 Keyboard data       |
 * | C      | 3   | I/O Open-drain | MS_DATA       | J15 pin 1  | PS/2 Mouse data          |
 * | C      | 4   | Output         | LPT_nINIT     | J13 pin 16 | Initialise Printer       |
 * | C      | 5   | Input          | LPT_SEL       | J13 pin 13 | Select (from printer)    |
 * | C      | 6   | Output         | LPT_nSELPRIN  | J13 pin 17 | Select (to printer)      |
 * | D      | 0   | Input          | UART_RX       | U1 PE1     | UART receive from MCU    |
 * | D      | 1   | Output         | UART_TX       | U1 PE0     | UART transmit to MCU     |
 * | D      | 2   | Input          | LPT_nACK      | J13 pin 10 | Acknowledge from Printer |
 * | D      | 3   | Input          | LPT_BUSY      | J13 pin 11 | Printer is Busy          |
 * | D      | 4   | Input          | LPT_nPE       | J13 pin 12 | Printer found Paper End  |
 * | D      | 5   | Input          | LPT_nERROR    | J13 pin 15 | Printer Error            |
 * | D      | 6   | Output         | LPT_nAUTOFEED | J13 pin 14 | Enable Auto Feed         |
 * | D      | 7   | Output         | LPT_nSTROBE   | J13 pin 1  | Latch data               |
 *
 * @author Jonathan Pallant <github@thejpster.org.uk>
 * @copyright 2019 Jonathan Pallant
 * @licence MIT or Apache 2.0 at your option.
 */

/**************************************************
* Includes
***************************************************/

#include <inttypes.h>
#include <stdbool.h>

#include <avr/interrupt.h>
#include <avr/io.h>
#include <avr/sleep.h>
#include <avr/wdt.h>

#include <avr-uart/uart.h>

#include "keypress.h"
#include "protocol.h"

/**************************************************
* Defines
***************************************************/

// Port B
#define LPT_D0 (1 << 0)
#define LPT_D1 (1 << 1)
#define LPT_D2 (1 << 2)
#define LPT_D3 (1 << 3)
#define LPT_D4 (1 << 4)
#define LPT_D5 (1 << 5)
#define LPT_D6 (1 << 6)
#define LPT_D7 (1 << 7)

// Port C
#define KB_CLK (1 << 0)
#define MS_CLK (1 << 1)
#define KB_DATA (1 << 2)
#define MS_DATA (1 << 3)
#define LPT_nINIT (1 << 4)
#define LPT_SEL (1 << 5)
#define LPT_nSELPRIN (1 << 6)

// Port D
#define UART_RX (1 << 0)
#define UART_TX (1 << 1)
#define LPT_nACK (1 << 2)
#define LPT_BUSY (1 << 3)
#define LPT_nPE (1 << 4)
#define LPT_nERROR (1 << 5)
#define LPT_nAUTOFEED (1 << 6)
#define LPT_nSTROBE (1 << 7)

#define OUR_UART_BAUD 115200UL

#define SHIFT_REGISTER_INIT_WORD (1 << 0)
#define SHIFT_REGISTER_COMPLETE_MASK (1 << 13)

#define MOUSE_BITS_OVERFLOW_Y (1 << 7)
#define MOUSE_BITS_OVERFLOW_X (1 << 6)
#define MOUSE_BITS_SIGN_Y (1 << 5)
#define MOUSE_BITS_SIGN_X (1 << 4)
#define MOUSE_BITS_1_BIT (1 << 3)
#define MOUSE_BITS_MIDDLE_BUTTON (1 << 2)
#define MOUSE_BITS_RIGHT_BUTTON (1 << 1)
#define MOUSE_BITS_LEFT_BUTTON (1 << 0)

#define LPT_BUFFER_SIZE 32

#define VERSION 0

#define soft_reset()           \
    do {                       \
        wdt_enable(WDTO_15MS); \
        for (;;) {             \
        }                      \
    } while (0)

/**************************************************
* Data Types
**************************************************/

/* None */

/**************************************************
* Function Prototypes
**************************************************/

static void setup_io(void);
static void process_command(uint8_t byte);
static void send_reset_cfm(void);
static void send_ps2_data_cfm(uint8_t kb0, uint8_t kb1, uint8_t kb2, uint8_t mouse_status, uint8_t mouse_x, uint8_t mouse_y);
static void send_ps2_led_cfm(void);
static void send_lpt_data_cfm(void);
static void send_lpt_read_cfm(void);
static void send_lpt_ctrl_cfm(void);
static void send_lpt_buffered_data_cfm(void);
static void send_lpt_read_pend_cfm(void);
static void send_lpt_set_mode_cfm(void);
static void send_ping_cfm(void);
static void send_bootloader_cfm(void);
static void send_booted_ind(bool keyboard, bool mouse, uint8_t version);
static void send_ps2_data_ind(void);
static void send_lpt_buffer_empty_ind(void);
static void send_lpt_read_pend_ind(uint8_t pins);
static void send_bad_command_ind(void);

void wdt_init(void) __attribute__((naked)) __attribute__((section(".init3")));

/**************************************************
* Public Data
**************************************************/

/* None */

/**************************************************
* Private Data
**************************************************/

/**
 * PS/2 data is clocked in as 13-bits into this 16-bit register. We default
 * with a 1 bit in the LSB. When this 1 bit has been shifted up to the 13-bit
 * we know we've clocked in a full word.
 */
static volatile uint16_t keyboard_word = SHIFT_REGISTER_INIT_WORD;
/**
 * PS/2 data is clocked in as 13-bits into this 16-bit register. We default
 * with a 1 bit in the LSB. When this 1 bit has been shifted up to the 13-bit
 * we know we've clocked in a full word.
 */
static volatile uint16_t mouse_word = SHIFT_REGISTER_INIT_WORD;

/**
 * Records the number of units the mouse has moved in the X direction since we
 * were last asked for the mouse position.
 */
static int16_t mouse_x = 0;

/**
 * Records the number of units the mouse has moved in the Y direction since we
 * were last asked for the mouse position.
 */
static int16_t mouse_y = 0;

/**
 * Records the mouse button status and overflow bits. See the MOUSE_BITS_xxx
 * macros.
 */
static uint8_t mouse_bits = 0;

/**
 * Buffers up to three keyboard output bytes. A value of 0x00 means (no keypress).
 */
static uint8_t keyboard_data[3] = { 0 };

/**
 * How many command bytes we're waiting for.
 */
static uint8_t command_bytes = 0;

/**
 * A buffer of bytes for the LPT to emit autonomously.
 */
volatile static uint8_t lpt_buffer[LPT_BUFFER_SIZE] = { 0 };
volatile static uint8_t lpt_buffer_num_bytes = 0;

/**************************************************
* Public Functions
***************************************************/

/**
 * Entry point to our program.
 *
 * @return Never
 */
int main(void)
{
    setup_io();
    // Scan for keyboard and mouse here
    bool mouse = false;
    bool keyboard = false;
    // Send booted ind
    send_booted_ind(keyboard, mouse, VERSION);
    uart_flush();
    // Wait for commands. We handle keyboard, mouse and LPT ack under
    // interrupt.
    for (;;) {
        sleep_mode();

        uint16_t status = uart0_getc();
        uint8_t data = status & 0x00FF;
        status >>= 8;
        switch (status) {
        case UART_NO_DATA:
            break;
        case 0:
            // Process received data
            process_command(data);
            break;
        case UART_BUFFER_OVERFLOW:
        case UART_OVERRUN_ERROR:
        case UART_FRAME_ERROR:
            // Uh-oh. Better reboot.
            break;
        default:
            break;
        }
    }
}

/**************************************************
* Private Functions
***************************************************/

/**
 * Configure the I/O pins correctly.
 */
void setup_io(void)
{
    DDRB = 0x00; // No inputs
    DDRC = KB_CLK | MS_CLK | KB_DATA | MS_DATA | LPT_nINIT | LPT_nSELPRIN;
    DDRD = LPT_nACK | LPT_BUSY | LPT_nPE | LPT_nERROR;
    PORTB = 0x00; // Outputs low by default
    PORTC = LPT_nINIT | LPT_nSELPRIN;
    PORTD = LPT_nACK | LPT_nPE | LPT_nERROR;

    // Configure interrupts here.
    // We need to interrupt on LPT_nACK and UART.

    uart0_init(UART_BAUD_SELECT(OUR_UART_BAUD, F_CPU));
}

/**
 * Process a byte from the MCU.
 */
static void process_command(uint8_t byte)
{
    if (command_bytes == 0) {
        // It's a new command
        switch (byte) {
        case PROTOCOL_RESET_REQ:
            send_reset_cfm();
            uart_flush();
            soft_reset();
            break;
        case PROTOCOL_PS2_DATA_REQ: {
            uint8_t kb0 = keyboard_data[0];
            uint8_t kb1 = keyboard_data[1];
            uint8_t kb2 = keyboard_data[2];
            uint8_t st = mouse_bits;
            uint8_t mx = 0;
            uint8_t my = 0;
            if ((mouse_x > 255) || (mouse_x < -255)) {
                st |= MOUSE_BITS_OVERFLOW_X;
                mx = 255;
            } else {
                mx = mouse_x > 0 ? mouse_x : -mouse_x;
            }
            st |= mouse_x > 0 ? 0 : MOUSE_BITS_SIGN_X;
            if ((mouse_y > 255) || (mouse_y < -255)) {
                st |= MOUSE_BITS_OVERFLOW_Y;
                my = 255;
            } else {
                my = mouse_y > 0 ? mouse_y : -mouse_y;
            }
            st |= mouse_y > 0 ? 0 : MOUSE_BITS_SIGN_Y;
            send_ps2_data_cfm(kb0, kb1, kb2, st, mx, my);
        } break;
        case PROTOCOL_PS2_LED_REQ:
            // send_ps2_led_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_DATA_REQ:
            // send_lpt_data_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_READ_REQ:
            // send_lpt_read_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_CTRL_REQ:
            // send_lpt_ctrl_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_BUFFERED_DATA_REQ:
            // send_lpt_buffered_data_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_READ_PEND_REQ:
            // send_lpt_read_pend_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_LPT_SET_MODE_REQ:
            // send_lpt_set_mode_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        case PROTOCOL_PING_REQ:
            send_ping_cfm();
            break;
        case PROTOCOL_BOOTLOADER_REQ:
            // send_bootloader_cfm();
            // Command unsupported
            send_bad_command_ind();
            break;
        default:
            send_bad_command_ind();
            break;
        }
    }
}

/**
 * Send a RESET_CFM to the MCU.
 */
static void send_reset_cfm(void)
{
    uart_putc(PROTOCOL_RESET_CFM);
}

/**
 * Send a PS2_DATA_CFM to the MCU.
 */
static void send_ps2_data_cfm(uint8_t kb0, uint8_t kb1, uint8_t kb2, uint8_t mouse_status, uint8_t mouse_x, uint8_t mouse_y)
{
    uart_putc(PROTOCOL_PS2_DATA_CFM);
    uart_putc(kb0);
    uart_putc(kb1);
    uart_putc(kb2);
    uart_putc(mouse_status);
    uart_putc(mouse_x);
    uart_putc(mouse_y);
}

/**
 * Send a PS2_LED_CFM to the MCU.
 */
static void send_ps2_led_cfm(void)
{
    uart_putc(PROTOCOL_PS2_LED_CFM);
}

/**
 * Send a LPT_DATA_CFM to the MCU.
 */
static void send_lpt_data_cfm(void)
{
    uart_putc(PROTOCOL_LPT_DATA_CFM);
}

/**
 * Send a LPT_READ_CFM to the MCU.
 */
static void send_lpt_read_cfm(void)
{
    uart_putc(PROTOCOL_LPT_READ_CFM);
}

/**
 * Send a LPT_CTRL_CFM to the MCU.
 */
static void send_lpt_ctrl_cfm(void)
{
    uart_putc(PROTOCOL_LPT_CTRL_CFM);
}

/**
 * Send a LPT_BUFFERED_DATA_CFM to the MCU.
 */
static void send_lpt_buffered_data_cfm(void)
{
    uart_putc(PROTOCOL_LPT_BUFFERED_DATA_CFM);
}

/**
 * Send a LPT_READ_PEND_CFM to the MCU.
 */
static void send_lpt_read_pend_cfm(void)
{
    uart_putc(PROTOCOL_LPT_READ_PEND_CFM);
}

/**
 * Send a LPT_SET_MODE_CFM to the MCU.
 */
static void send_lpt_set_mode_cfm(void)
{
    uart_putc(PROTOCOL_LPT_SET_MODE_CFM);
}

/**
 * Send a PING_CFM to the MCU.
 */
static void send_ping_cfm(void)
{
    uart_putc(PROTOCOL_PING_CFM);
}

/**
 * Send a BOOTLOADER_CFM to the MCU.
 */
static void send_bootloader_cfm(void)
{
    uart_putc(PROTOCOL_BOOTLOADER_CFM);
}

/**
 * Send a BOOTED_IND (0xF3)
 *
 * @param keyboard true if keyboard present, false otherwise
 * @param mouse    true if mouse present, false otherwise
 * @param version  the firmware release version to report
 */
static void send_booted_ind(bool keyboard, bool mouse, uint8_t version)
{
    uart_putc(PROTOCOL_BOOTED_IND);
    uart_putc((keyboard << 0) | (mouse << 1) | (version << 2) | (1 << 7));
}

/**
 * Sends a PS2_DATA_IND (0xF1). Indicates that data is waiting to be read.
 * Only sent once, until a PS2_DATA_REQ is seen.
 */
static void send_ps2_data_ind(void)
{
    uart_putc(PROTOCOL_PS2_DATA_IND);
}

/**
 * Send LPT_BUFFER_EMPTY_IND (0xF2) - Indicates that the LPT buffer is now empty and   more bytes can be sent.
 */
static void send_lpt_buffer_empty_ind(void)
{
    uart_putc(PROTOCOL_LPT_BUFFER_EMPTY_IND);
}

/**
 * Send LPT_READ_PEND_IND (0xF3) <pins> - The input pins matched the given levels.
 */
static void send_lpt_read_pend_ind(uint8_t pins)
{
    uart_putc(PROTOCOL_LPT_READ_PEND_IND);
    uart_putc(pins);
}

/**
 * Send BAD_COMMAND_IND (0xF4) - Send when a bad request is received. That request   will not receive a Confirmation.
 */
static void send_bad_command_ind(void)
{
    uart_putc(PROTOCOL_BAD_COMMAND_IND);
}

/**
 * Disable watchdog on boot.
 */
void wdt_init(void)
{
    MCUSR = 0;
    wdt_disable();
}

/**************************************************
* End of file
***************************************************/
