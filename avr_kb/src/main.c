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

#ifdef __AVR__
#include <avr/interrupt.h>
#include <avr/io.h>
#include <avr/sleep.h>
#include <avr/wdt.h>
#include <avr-uart/uart.h>
#else
#define _BV(bit) (1 << (bit))
#include <stdio.h>
#include <stdlib.h>
#include "fake_uart.h"
#endif
#include "keypress.h"
#include "keycodes.h"
#include "protocol.h"

/**************************************************
* Defines
***************************************************/

// Port B
#define PB_LPT_D0 _BV(0)
#define PB_LPT_D1 _BV(1)
#define PB_LPT_D2 _BV(2)
#define PB_LPT_D3 _BV(3)
#define PB_LPT_D4 _BV(4)
#define PB_LPT_D5 _BV(5)
#define PB_LPT_D6 _BV(6)
#define PB_LPT_D7 _BV(7)

// Port C
#define PC_KB_CLK _BV(0)
#define PC_MS_CLK _BV(1)
#define PC_KB_DATA _BV(2)
#define PC_MS_DATA _BV(3)
#define PC_LPT_nINIT _BV(4)
#define PC_LPT_SEL _BV(5)
/**
 * This clashes with Reset. On some chips the RSTDISBL fuse isn't set and so
 * this pin won't function as a GPIO - the printer side should therefore be
 * tied low in hardware so the printer is always selected. Where RSTDISBL is
 * set, this pin can be connected to the printer, but you can re-program the
 * chip with an ISP.
*/
#define PC_LPT_nSELPRIN _BV(6)

// Port D
#define PD_UART_RX _BV(0)
#define PD_UART_TX _BV(1)
#define PD_LPT_nACK _BV(2)
#define PD_LPT_BUSY _BV(3)
#define PD_LPT_nPE _BV(4)
#define PD_LPT_nERROR _BV(5)
#define PD_LPT_nAUTOFEED _BV(6)
#define PD_LPT_nSTROBE _BV(7)

#define OUR_UART_BAUD 115200UL

#define SHIFT_REGISTER_INIT_WORD _BV(0)
#define SHIFT_REGISTER_COMPLETE_MASK _BV(12)

#define EXTENDED_KEY_CODE 0xE0
#define RELEASE_KEY_CODE 0xF0

#define MOUSE_BITS_OVERFLOW_Y _BV(7)
#define MOUSE_BITS_OVERFLOW_X _BV(6)
#define MOUSE_BITS_SIGN_Y _BV(5)
#define MOUSE_BITS_SIGN_X _BV(4)
#define MOUSE_BITS_1_BIT _BV(3)
#define MOUSE_BITS_MIDDLE_BUTTON _BV(2)
#define MOUSE_BITS_RIGHT_BUTTON _BV(1)
#define MOUSE_BITS_LEFT_BUTTON _BV(0)

#define LPT_BUFFER_SIZE 32

/** Same as the TM4C123 */
#define UART_RX_BUFFER_SIZE 16

/** Same as the TM4C123 */
#define UART_TX_BUFFER_SIZE 16

#define VERSION 0

#ifdef __AVR__
#define soft_reset()           \
    do {                       \
        wdt_enable(WDTO_15MS); \
        for (;;) {             \
        }                      \
    } while (0)
#define run() 1
#else
static uint8_t should_run = 1;
#define soft_reset() do { should_run = 0; } while(0)
#define run() should_run
#define sleep_mode() do { } while(0)
#endif

/**************************************************
* Data Types
**************************************************/

typedef struct fifo_t {
    volatile uint8_t* p_data;
    uint8_t size;
    volatile uint8_t head;
    volatile uint8_t tail;
    volatile uint8_t dropped_bytes;
} fifo_t;

typedef enum bit_state_t {
    BIT_STATE_LOW,
    BIT_STATE_HIGH,
    BIT_STATE_WAITING
} bit_state_t;

typedef enum keyboard_state_t {
    KEYBOARD_STATE_IDLE,
    KEYBOARD_STATE_EXTENDED,
    KEYBOARD_STATE_EXTENDED_RELEASE,
    KEYBOARD_STATE_RELEASE
} keyboard_state_t;

/**************************************************
* Function Prototypes
**************************************************/

static void setup_io(void);
static bool process_port_serial(void);
static bool process_port_keyboard(void);
static bool process_port_mouse(void);
static bool process_port_parallel(void);
static void process_command(uint8_t byte);
static bool decode_word(uint16_t word, uint8_t* p_output_byte);
static void store_keyboard_byte(bool is_keydown, uint8_t byte);
static void store_keyboard_extended_byte(bool is_keydown, uint8_t byte);
static void store_keyevent(uint8_t byte);
static void send_reset_cfm(void);
static void send_ps2_data_cfm(uint8_t kb0, uint8_t kb1, uint8_t kb2, uint8_t mouse_status, uint8_t mouse_x, uint8_t mouse_y);
static void send_ps2_led_cfm(void);
static void send_lpt_data_cfm(void);
static void send_lpt_read_cfm(void);
static void send_lpt_ctrl_cfm(void);
static void send_lpt_buffered_data_cfm(uint8_t status);
static void send_lpt_read_pend_cfm(void);
static void send_lpt_set_mode_cfm(void);
static void send_ping_cfm(void);
static void send_bootloader_cfm(void);
static void send_booted_ind(bool keyboard, bool mouse, uint8_t version);
static void send_ps2_data_ind(void);
static void send_lpt_buffer_empty_ind(void);
static void send_lpt_read_pend_ind(uint8_t pins);
static void send_bad_command_ind(void);

#ifdef __AVR__
void wdt_init(void) __attribute__((naked)) __attribute__((section(".init3")));
#endif

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
static uint16_t g_keyboard_word;

/**
 * PS/2 data is clocked in as 13-bits into this 16-bit register. We default
 * with a 1 bit in the LSB. When this 1 bit has been shifted up to the 13-bit
 * we know we've clocked in a full word.
 */
static uint16_t g_mouse_word;

/**
 * Records the number of units the mouse has moved in the X direction since we
 * were last asked for the mouse position.
 */
static int16_t g_mouse_x = 0;

/**
 * Records the number of units the mouse has moved in the Y direction since we
 * were last asked for the mouse position.
 */
static int16_t g_mouse_y = 0;

/**
 * Records the mouse button status and overflow bits. See the MOUSE_BITS_xxx
 * macros.
 */
static uint8_t g_mouse_bits = 0;

/**
 * Buffers up to three keypress event bytes. A value of 0x00 means (no
 * keypress).
 */
static uint8_t g_keyboard_data[3] = { 0 };

/**
 * Tracks the keyboard state.
 */
static keyboard_state_t g_keyboard_state = KEYBOARD_STATE_IDLE;

/**
 * How many command bytes we're waiting for.
 */
static uint8_t g_command_bytes = 0;

/**
 * A buffer of bytes for the LPT to emit autonomously.
 */
volatile static uint8_t g_lpt_buffer[LPT_BUFFER_SIZE] = { 0 };
static fifo_t g_lpt_fifo = { 0 };

/**
 * The previous value of the PS/2 pins in port C, so we
 * can tell what changed when the interrupt fires.
 */
static volatile uint8_t g_pc_previous = 0;

/**
 * The last PS/2 mouse bit received.
 */
static volatile bit_state_t g_ms_bit = BIT_STATE_WAITING;

/**
 * The last PS/2 keyboard bit received.
 */
static volatile bit_state_t g_kb_bit = BIT_STATE_WAITING;


/**************************************************
* Public Functions
***************************************************/

/**
 * Entry point to our program.
 *
 * @return Never returns on AVR. Return on RESET_REQ on Linux.
 */
int main(void)
{
    g_lpt_fifo.p_data = g_lpt_buffer;
    g_lpt_fifo.size = sizeof(g_lpt_buffer);
    g_keyboard_word = SHIFT_REGISTER_INIT_WORD;
    g_mouse_word = SHIFT_REGISTER_INIT_WORD;
    DDRB = 0xFF;
    PORTB = 0xAA;
    volatile uint8_t test = 0;
    for(;;) {
        test++;
    }

    // setup_io();
    // bool mouse = false;
    // bool keyboard = false;
    // // Scan for keyboard and mouse here
    // // TODO
    // // Send booted ind
    // send_booted_ind(keyboard, mouse, VERSION);
    // uart_flush();
    // // Wait for commands. We handle keyboard, mouse and LPT ack under
    // // interrupt.
    // uint8_t leds = 0;
    // uint16_t counter = 0;
    // while(run()) {
    //     if (counter++ == 0) {
    //         PORTB = ++leds;
    //     }
    //     bool more_processing = false;
    //     do {
    //         more_processing |= process_port_keyboard();
    //         more_processing |= process_port_mouse();
    //         more_processing |= process_port_parallel();
    //         more_processing |= process_port_serial();
    //     } while(more_processing);
    //     sleep_mode();
    // }
}

/**
 * Pin-change interrupt for Port C.
 */
ISR(PCINT1_vect)
{
    uint8_t pins = PINC & (PC_KB_CLK | PC_KB_DATA | PC_MS_CLK | PC_MS_DATA);
    uint8_t changed = g_pc_previous ^ pins;
    if ((changed & PC_KB_CLK) && ((pins & PC_KB_CLK) == 0)) {
        // Falling edge on KB_CLK
        g_kb_bit = (pins & PC_KB_DATA);
    }
    if ((changed & PC_MS_CLK) && ((pins & PC_MS_CLK) == 0)) {
        // Falling edge on MS_CLK
        g_ms_bit = (pins & PC_MS_DATA);
    }
    g_pc_previous = pins; // Save the previous state so you can tell what changed
}

// #ifdef __AVR__
// /**
//  * UART0 receive interrupt service routine. Copies byte from UART data
//  * register to FIFO. If FIFO is full, byte is dropped.
//  */
// ISR( USART_RX_vect )
// {
//     uint8_t head = uart_rx_fifo.head;
//     uint8_t tail = uart_rx_fifo.tail;
//     head = head + 1;
//     if (head == uart_rx_fifo.size) {
//         head = 0;
//     }
//     if (head == tail) {
//         // FIFO is full, so we drop the byte and increment a counter
//         uart_rx_fifo.dropped_bytes++;
//     } else {
//         uart_rx_fifo.p_data[head] = UDR0;
//         uart_rx_fifo.head = head;
//     }
// }
// #endif

/**************************************************
* Private Functions
***************************************************/

/**
 * Configure the I/O pins correctly.
 */
static void setup_io(void)
{
#ifdef __AVR__
    // 1 = output, 0 = input
    DDRB = 0xFF; // All outputs
    DDRC = PC_LPT_nINIT | PC_LPT_nSELPRIN;
    DDRD = PD_UART_TX | PD_LPT_nAUTOFEED | PD_LPT_nSTROBE;
    PORTB = 0xFF; // Outputs high by default
    PORTC = PC_LPT_nINIT | PC_LPT_nSELPRIN;
    PORTD = PD_LPT_nSTROBE | PD_LPT_nAUTOFEED;
#endif

    // Configure interrupts here.
    // We need to interrupt on LPT_nACK and UART.

    uart_init(UART_BAUD_SELECT(OUR_UART_BAUD, F_CPU));
}

/**
 * Handle the Serial port.
 * @return true if more processing required, false if safe to sleep.
 */
static bool process_port_serial(void)
{
    bool result = true;
    uint16_t status = uart_getc();
    uint8_t data = status & 0x00FF;
    switch (status & 0xFF00) {
    case UART_NO_DATA:
        result = false;
        break;
    case 0:
        // Process received data
        process_command(data);
        break;
    case UART_BUFFER_OVERFLOW:
    case UART_OVERRUN_ERROR:
    case UART_FRAME_ERROR:
        // Uh-oh. Better reboot.
        soft_reset();
        break;
    default:
        soft_reset();
        break;
    }
    return result;
}

/**
 * Handle the Keyboard port.
 * @return true if more processing required, false if safe to sleep.
 */
static bool process_port_keyboard(void)
{
    bit_state_t state = g_kb_bit;
    if (state != BIT_STATE_WAITING) {
        uint16_t word = g_keyboard_word;
        word <<= 1;
        if (state == BIT_STATE_HIGH) {
            word |= 1;
        } else {
            word <<= 1;
        }
        if (word & SHIFT_REGISTER_COMPLETE_MASK) {
            uint8_t data = 0;
            if (decode_word(word, &data)) {
                word = SHIFT_REGISTER_INIT_WORD;
                switch (g_keyboard_state) {
                    case KEYBOARD_STATE_IDLE:
                    {
                        switch (data) {
                            case EXTENDED_KEY_CODE:
                                g_keyboard_state = KEYBOARD_STATE_EXTENDED;
                                break;
                            case RELEASE_KEY_CODE:
                                g_keyboard_state = KEYBOARD_STATE_RELEASE;
                                break;
                            default:
                                store_keyboard_byte(false, data);
                                break;
                        }
                    }
                    break;
                    case KEYBOARD_STATE_EXTENDED:
                    {
                        store_keyboard_extended_byte(false, data);
                    }
                    break;
                    case KEYBOARD_STATE_EXTENDED_RELEASE:
                    {
                        store_keyboard_extended_byte(true, data);
                    }
                    break;
                    case KEYBOARD_STATE_RELEASE:
                    {
                        switch (data) {
                            case EXTENDED_KEY_CODE:
                                g_keyboard_state = KEYBOARD_STATE_EXTENDED;
                                break;
                            default:
                                store_keyboard_byte(true, data);
                                break;
                        }
                    }
                    break;
                }
            }
        }
        g_keyboard_word = word;
    }
    return false;
}

/**
 * Handle the Mouse port.
 * @return true if more processing required, false if safe to sleep.
 */
static bool process_port_mouse(void)
{
    return false;
}

/**
 * Handle the Parallel port.
 * @return true if more processing required, false if safe to sleep.
 */
static bool process_port_parallel(void)
{
    return false;
}


/**
 * Process a byte from the MCU.
 */
static void process_command(uint8_t byte)
{
    if (g_command_bytes == 0) {
        // It's a new command
        switch (byte) {
        case PROTOCOL_RESET_REQ:
            send_reset_cfm();
            uart_flush();
            soft_reset();
            break;
        case PROTOCOL_PS2_DATA_REQ: {
            uint8_t kb0 = g_keyboard_data[0];
            uint8_t kb1 = g_keyboard_data[1];
            uint8_t kb2 = g_keyboard_data[2];
            uint8_t st = g_mouse_bits;
            uint8_t mx = 0;
            uint8_t my = 0;
            if ((g_mouse_x > 255) || (g_mouse_x < -255)) {
                st |= MOUSE_BITS_OVERFLOW_X;
                mx = 255;
            } else {
                mx = g_mouse_x > 0 ? g_mouse_x : -g_mouse_x;
            }
            st |= g_mouse_x > 0 ? 0 : MOUSE_BITS_SIGN_X;
            if ((g_mouse_y > 255) || (g_mouse_y < -255)) {
                st |= MOUSE_BITS_OVERFLOW_Y;
                my = 255;
            } else {
                my = g_mouse_y > 0 ? g_mouse_y : -g_mouse_y;
            }
            st |= g_mouse_y > 0 ? 0 : MOUSE_BITS_SIGN_Y;
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
 * Convert a 11-bit word (start, 8-data bits, parity, stop) to an 8-bit byte.
 *
 * @return true if word OK, false otherwise
 */
static bool decode_word(uint16_t word, uint8_t* p_output_byte) {
    // Stop bit
    if ((word & _BV(10)) == 0) {
        return false;
    }
    // Start bit
    if ((word & _BV(0)) != 0) {
        return false;
    }
    // TODO check parity _BV(9) here!
    *p_output_byte = (uint8_t) ((word >> 1) & 0xFF);
    return true;
}

/**
 * Handle a scan-code set 2 keyboard byte, storing it in the key event buffer.
 */
static void store_keyboard_byte(bool is_keydown, uint8_t byte) {
    uint8_t up_down_bit = is_keydown ? KEYPRESS_KEYDOWN : KEYPRESS_KEYUP;
    if (byte <= KEYCODE_SCS2_SCROLLLOCK) {
        // As a short-cut, these scan-code 2 code map straight through
        store_keyevent(byte | up_down_bit);
    } else if (byte == KEYCODE_SCS2_F7) {
        // This is the only one that doesn't
        store_keyevent(KEYPRESS_F7 | up_down_bit);
    }
}

/**
 * Handle a extended scan-code set 2 keyboard byte, storing it in the key
 * event buffer.
 */
static void store_keyboard_extended_byte(bool is_keydown, uint8_t byte) {
    uint8_t up_down_bit = is_keydown ? KEYPRESS_KEYDOWN : KEYPRESS_KEYUP;
    uint8_t data = 0;
    switch (byte) {
    case KEYCODE_SCS2_EXTENDED_ALTRIGHT:
        data = KEYPRESS_ALTRIGHT;
        break;
    case KEYCODE_SCS2_EXTENDED_CONTROLRIGHT:
        data = KEYPRESS_CONTROLRIGHT;
        break;
    case KEYCODE_SCS2_EXTENDED_WINDOWSLEFT:
        data = KEYPRESS_WINDOWSLEFT;
        break;
    case KEYCODE_SCS2_EXTENDED_WINDOWSRIGHT:
        data = KEYPRESS_WINDOWSRIGHT;
        break;
    case KEYCODE_SCS2_EXTENDED_MENUS:
        data = KEYPRESS_MENUS;
        break;
    case KEYCODE_SCS2_EXTENDED_NUMPADSLASH:
        data = KEYPRESS_NUMPADSLASH;
        break;
    case KEYCODE_SCS2_EXTENDED_NUMPADENTER:
        data = KEYPRESS_NUMPADENTER;
        break;
    case KEYCODE_SCS2_EXTENDED_END:
        data = KEYPRESS_END;
        break;
    case KEYCODE_SCS2_EXTENDED_ARROWLEFT:
        data = KEYPRESS_ARROWLEFT;
        break;
    case KEYCODE_SCS2_EXTENDED_HOME:
        data = KEYPRESS_HOME;
        break;
    case KEYCODE_SCS2_EXTENDED_INSERT:
        data = KEYPRESS_INSERT;
        break;
    case KEYCODE_SCS2_EXTENDED_DELETE:
        data = KEYPRESS_DELETE;
        break;
    case KEYCODE_SCS2_EXTENDED_ARROWDOWN:
        data = KEYPRESS_ARROWDOWN;
        break;
    case KEYCODE_SCS2_EXTENDED_ARROWRIGHT:
        data = KEYPRESS_ARROWRIGHT;
        break;
    case KEYCODE_SCS2_EXTENDED_ARROWUP:
        data = KEYPRESS_ARROWUP;
        break;
    case KEYCODE_SCS2_EXTENDED_PAGEDOWN:
        data = KEYPRESS_PAGEDOWN;
        break;
    case KEYCODE_SCS2_EXTENDED_PAGEUP:
        data = KEYPRESS_PAGEUP;
        break;
    default:
        break;
    }
    if (data != 0) {
        store_keyevent(data | up_down_bit);
    }
}

/**
 * Buffer a key event for delivery.
 */
static void store_keyevent(uint8_t byte) {
    if (g_keyboard_data[0] == 0) {
        g_keyboard_data[0] = byte;
    } else if (g_keyboard_data[1] == 0) {
        g_keyboard_data[1] = byte;
    } else if (g_keyboard_data[2] == 0) {
        g_keyboard_data[2] = byte;
    } else {
        // We dropped a key event. Oops.
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
static void send_lpt_buffered_data_cfm(uint8_t status)
{
    uart_putc(PROTOCOL_LPT_BUFFERED_DATA_CFM);
    uart_putc(status);
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
 * Send LPT_BUFFER_EMPTY_IND (0xF2) - Indicates that the LPT buffer is now
 * empty and more bytes can be sent.
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
 * Send BAD_COMMAND_IND (0xF4) - Send when a bad request is received. That
 * request   will not receive a Confirmation.
 */
static void send_bad_command_ind(void)
{
    uart_putc(PROTOCOL_BAD_COMMAND_IND);
}

/**
 * Disable watchdog on boot.
 */
#ifdef __AVR__
void wdt_init(void)
{
    MCUSR = 0;
    wdt_disable();
}
#endif

/**************************************************
* End of file
***************************************************/
