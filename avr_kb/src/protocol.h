/** @file protocol.h
 *
 * Protocol definitions for the MCU to IO protocol.
 *
 * @author Jonathan Pallant <github@thejpster.org.uk>
 * @copyright 2019 Jonathan Pallant
 * @licence MIT or Apache 2.0 at your option.
 */

#define PROTOCOL_RESET_REQ 'A'
#define PROTOCOL_PS2_DATA_REQ 'B'
#define PROTOCOL_PS2_LED_REQ 'C'
#define PROTOCOL_LPT_DATA_REQ 'D'
#define PROTOCOL_LPT_READ_REQ 'E'
#define PROTOCOL_LPT_CTRL_REQ 'F'
#define PROTOCOL_LPT_BUFFERED_DATA_REQ 'G'
#define PROTOCOL_LPT_READ_PEND_REQ 'H'
#define PROTOCOL_LPT_SET_MODE_REQ 'I'
#define PROTOCOL_PING_REQ 'J'
#define PROTOCOL_BOOTLOADER_REQ 'K'

#define PROTOCOL_RESET_CFM 'a'
#define PROTOCOL_PS2_DATA_CFM 'b'
#define PROTOCOL_PS2_LED_CFM 'c'
#define PROTOCOL_LPT_DATA_CFM 'd'
#define PROTOCOL_LPT_READ_CFM 'e'
#define PROTOCOL_LPT_CTRL_CFM 'f'
#define PROTOCOL_LPT_BUFFERED_DATA_CFM 'g'
#define PROTOCOL_LPT_READ_PEND_CFM 'h'
#define PROTOCOL_LPT_SET_MODE_CFM 'i'
#define PROTOCOL_PING_CFM 'j'
#define PROTOCOL_BOOTLOADER_CFM 'k'

#define PROTOCOL_BOOTED_IND '0'
#define PROTOCOL_PS2_DATA_IND '1'
#define PROTOCOL_LPT_BUFFER_EMPTY_IND '2'
#define PROTOCOL_LPT_READ_PEND_IND '3'
#define PROTOCOL_BAD_COMMAND_IND '4'

/**************************************************
* End of file
***************************************************/
