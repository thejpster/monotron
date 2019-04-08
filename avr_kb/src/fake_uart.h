/** @file fake_uart.h
 *
 * Implements the avr-uart API but using stdin and stdout.
 *
 * @author Jonathan Pallant <github@thejpster.org.uk>
 * @copyright 2019 Jonathan Pallant
 * @licence MIT or Apache 2.0 at your option.
 */

#define UART_FRAME_ERROR      0x0800              /**< Framing Error by UART       */
#define UART_OVERRUN_ERROR    0x0400              /**< Overrun condition by UART   */
#define UART_BUFFER_OVERFLOW  0x0200              /**< receive ringbuffer overflow */
#define UART_NO_DATA          0x0100              /**< no receive data available   */

#define UART_BAUD_SELECT(b, clk) (b)

void uart_init(unsigned int baud);
uint16_t uart_getc(void);
uint16_t uart_peek(void);
void uart_putc(uint8_t d);
void uart_puts(const char* s);
void uart_puts_p(const char* s);
uint16_t uart_available(void);
void uart_flush(void);

/**************************************************
* End of file
***************************************************/
