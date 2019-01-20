/** @file fake_uart.c
 *
 * Implements the AVR-UART API using stdio so we can
 * test on a real PC.
 *
 * @author Jonathan Pallant <github@thejpster.org.uk>
 * @copyright 2019 Jonathan Pallant
 * @licence MIT or Apache 2.0 at your option.
 */

/**************************************************
* Includes
***************************************************/

#define _POSIX_C_SOURCE 200112L

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <termios.h>
#include <stdlib.h>
#include <time.h>
#include <fcntl.h>
#include <signal.h>
#include "fake_uart.h"

/**************************************************
* Defines
***************************************************/

/* None */

/**************************************************
* Data Types
**************************************************/

/* None */

/**************************************************
* Function Prototypes
**************************************************/

static void setup_console(void);
static void clean_up(void);
static void sigint_handler(int dummy);
static int kbhit(void);

/**************************************************
* Public Data
**************************************************/

/* None */

/**************************************************
* Private Data
**************************************************/

static struct termios oldt;

/**************************************************
* Public Functions
***************************************************/

void uart_init(unsigned int _baud) {
    setup_console();
    signal(SIGINT, sigint_handler);
    atexit(clean_up);
}

uint16_t uart_getc(void) {
    if (kbhit()) {
        return fgetc(stdin) & 0xFF;
    } else {
        return 0x0100;
    }
}

uint16_t uart_peek(void) {
    if (kbhit()) {
        int ch = fgetc(stdin);
        ungetc(ch, stdin);
        return ch & 0xFF;
    } else {
        return 0x0100;
    }
}

void uart_putc(uint8_t d) {
    putchar(d);
}

void uart_puts(const char* s) {
    while(*s) {
        putchar(*s++);
    }
}

void uart_puts_p(const char* s) {
    while(*s) {
        putchar(*s++);
    }
}

uint16_t uart_available(void) {
    if (kbhit()) {
        return 1;
    } else {
        return 0;
    }
}

void uart_flush(void) {
    fflush(stdout);
}


/**************************************************
* Private Functions
***************************************************/

static void setup_console(void) {
    // Disable echo
    struct termios newt;
    int fd_stdin = fileno(stdin);
    tcgetattr(fd_stdin, &oldt);
    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(fd_stdin, TCSANOW, &newt);
}

static void clean_up(void) {
    int fd_stdin = fileno(stdin);
    tcsetattr(fd_stdin, TCSANOW, &oldt);
    printf("\e[?25h\e[0m\e[2J");
    exit(0);
}

static void sigint_handler(int dummy) {
    ungetc(0x03, stdin);
}

/**
 * @return 1 if char available, 0 otherwise
 */
static int kbhit(void)
{
    struct termios oldt;
    int fd_stdin = fileno(stdin);
    tcgetattr(fd_stdin, &oldt);
    struct termios newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(fd_stdin, TCSANOW, &newt);
    int oldf = fcntl(fd_stdin, F_GETFL, 0);
    fcntl(fd_stdin, F_SETFL, oldf | O_NONBLOCK);
    int ch = getchar();
    tcsetattr(fd_stdin, TCSANOW, &oldt);
    fcntl(fd_stdin, F_SETFL, oldf);
    if(ch != EOF)
    {
        ungetc(ch, stdin);
        return 1;
    }
    return 0;
}


/**************************************************
* End of file
***************************************************/
