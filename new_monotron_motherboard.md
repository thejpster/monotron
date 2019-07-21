# New Monotron Motherboard Design

I will produce a basic Monotron motherboard, with SPI/I2C/UART based expansion
slots. On-board functionality is limited to Audio and Sound - add in cards are
required for user input (via Bluetooth, PS/2 or USB), and for communication
(WiFi, Bluetooth, RS232) and Storage (IDE, USB Host, SD card).

In future, other motherboards could exist - e.g. an STM32F4 or F7 based one,
or even a Z80 based one with TMS9918 video controller and AY-3-8910 sound
chip.

## Expansion Slot Pinout

All signals are at 3.3V levels, but VCC_3.3V is limited to 300mA total across
the bus. If you need more current, add a 3.3V LDO to your expansion board.

1. VCC_5V
2. VCC_3.3V
3. UART_RX
4. UART_TX
5. UART_RTS (optional - not on all slots)
6. UART_CTS (optional - not on all slots)
7. SPI_BUS_CLK
8. Ground
9. SPI_BUS_MOSI
10. Ground
11. SPI_BUS_MISO
12. Ground
13. SPI_CS (unique per slot)
14. Ground
15. I2C_BUS_SDA
16. Ground
17. I2C_BUS_SCL
18. Ground
19. IRQ (unique per slot)
20. Ground

For the Tiva-C Launchpad motherboard, that works out as:

1. VCC_5V
2. VCC_3.3V
3. UART_RX (PB0, PC6)
4. UART_TX (PB1, PC7)
5. UART_RTS (PC4)
6. UART_CTS (PC5)
7. SPI_BUS_CLK (PA2)
8. Ground
9. SPI_BUS_MOSI (PA5)
10. Ground
11. SPI_BUS_MISO (PA4)
12. Ground
13. SPI_CS (PA3 PD2 PD6 PD7)
14. Ground
15. I2C_BUS_SDA (PA7)
16. Ground
17. I2C_BUS_SCL (PA6)
18. Ground
19. IRQ (PB3 PE2 PE3 PF4)
20. Ground


Four external slots, as above. Motherboard only has VGA and audio (because
they're too fast to run over the expansion bus).

## Proposed Expansion Cards

* 2x Atari Joystick (MCP23017 based)
* 2x Sega Megadrive pad (Atmega?)
* Parallel Printer Port (Atmega? MCP23017?)
* Dual PS/2 (Atmega?)
* Full 9-wire RS232 (Atmega?)
* WiFi (ESP8266 on-chip stack)
* WiFi + Bluetooth (ESP32  on-chip stack)
* 10base-T Ethernet (Microchip 28J60, MAC/PHY only)
* PmodNIC100 based 100base-T Ethernet (Microchip ENC424J600, MAC/PHY only)
* USB Serial Device (FT232, with USB micro-B socket)
* Full-size SD Card slot
* Micro SD slot
* Real Time Clock (I2C)
* IDE interface (Atmega, see http://sbc.rictor.org/io/IDE.html)
* USB Host interface (MAX3421E, see https://www.sparkfun.com/products/9947)
* Keyboard/Mouse/Serial/Parallel controller - with some
  connectors on separate expansion plate via ribbon (NCT6686D based, http://www.nuvoton.com/resource-files/NCT6686D_HW_Datasheet_V0_5.pdf)
* Raspberry Pi HAT adaptor (e.g. for the SenseHAT)
* SPI to ISA bridge (turns microATX board into full-size ATX)

## Back Panel layout

```
                                        +---+  +---+  +---+  +---+
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
+----+  +-------+                       |   |  |   |  |   |  |   |
|/--\|  | ..... |   +---+               |   |  |   |  |   |  |   |
||__||  | ::::: |   | O |               |   |  |   |  |   |  |   |
+----+--+-------+---+---+---------------+---+--+---+--+---+--+---+
 Power   Monitor   Line-Out             Slot0  Slot1  Slot2  Slot3
 USB B
```

## Pins which stay on the motherboard.

* VGA_HSYNC (PB4)
* VGA_VSYNC (PB5)
* VGA_R (PF1)
* VGA_G (PB7)
* VGA_B (PD3)
* KEYB_UART_RX (PE0)
* KEYB_UART_TX (PE1)
* AUDIO_LEFT (PE4)
* AUDIO_RIGHT (PE5)

## Labels for the slots

UART1 (4-wire)
SPI CS0 / IRQ 0

UART1 (4-wire)
SPI CS1 / IRQ 1

UART3 (2-wire)
SPI CS2 / IRQ 2

UART3 (2-wire)
SPI CS3 / IRQ 3

You can only one UART 4-wire board fitted (in slot 0 or slot 1) and one UART
2-wire board fitted (in slot 2 or slot 3). You can have four SPI boards
or four I2C boards fitted at once.
