/** @file main.c
 *
 * I/O Controller Firmware for Monotron.
 *
 * See README.md.
 *
 * @author Jonathan Pallant <github@thejpster.org.uk>
 * @copyright 2019 Jonathan Pallant
 * @licence MIT or Apache 2.0 at your option.
 */

#include <avr/interrupt.h>
#include <avr/io.h>
#include <avr/sleep.h>
#include <inttypes.h>

/**
 * Entry point to our program.
 */
int main(void)
{
    for (;;) {
        sleep_mode();
    }
}

// End of file