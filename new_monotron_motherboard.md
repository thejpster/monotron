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
1. VCC_3.3V
1. VCC_5V
1. VCC_3.3V
1. UART_RX
1. SPI_MOSI
1. UART_TX
1. SPI_MISO
1. UART_CTS (optional - not on all slots)
1. SPI_CLK
1. UART_RTS (optional - not on all slots)
1. Not Connected (reserved for future use)
1. SPI_CS (unique per slot)
1. I2C_BUS_SDA
1. IRQ (unique per slot)
1. I2C_BUS_SCL
1. Ground
1. Ground
1. Ground
1. Ground

The expansion board can have up to four external slots. The CPU Board only has
VGA, Audio and maybe USB (because they're too fast to run over the expansion
bus).

Alternative format expansion boards might be available that, for example, give
you a couple of mikroelectronik Clik(tm) slots, or an Arduino shield interface
(albeit with 3.3V signalling).

## Possible Expansion Cards

* Joysticks
	* 2x Atari Joystick (MCP23017 + SPI)
	* 2x Sega Megadrive pad (Atmega?)
	* 4-channel ADC to support 2x Analog PC Joysticks on a DB15
* Parallel Printer Port
	* Atmega 328 + UART/I2C/SPI, or
	* MCP23017 + SPI (one pin short - could use IRQ line as extra input?)
	* MCP23S17 + I2C (one pin short - could use IRQ line as extra input?)
* Dual PS/2
	* Atmega 328 + UART/I2C/SPI, or
	* SMBus PS/2 keyboard controller
	* Keyboard/Mouse/Serial/Parallel controller - with some
	  connectors on separate expansion plate via ribbon (NCT6686D based, http://www.nuvoton.com/resource-files/NCT6686D_HW_Datasheet_V0_5.pdf)
* UART
	* I2C or SPI to full 9-wire RS-232
	* RS-232 level shifter on 5-wire UART (no DCD, DTR, DSR or RI)
	* MIDI In/Out (might need DB15 to 2xDIN cable), using 2-wire UART
		* Could combine with DB15 Game Port
* USB
	* Communications Class Device
		* FT232 with USB micro-B or USB B socket + 4-wire UART
	* Host
		* Off-chip interface (MAX3421E + SPI, see https://www.sparkfun.com/products/9947)
		* Use Tiva-C OTG support, with separate USB Hub (e.g. MaxLinear XR22404CG28TR-F)
* Networking
	* WiFi
		* ESP8266 on-chip stack + UART
		* ESP32 on-chip stack + UART (also has Bluetooth)
	* Ethernet
		* Microchip 28J60 + SPI, MAC/PHY only
		* Microchip ENC424J600 + SPI, MAC/PHY only (PmodNIC100 board)
	* Bluetooth
		* UART HCI device
		* UART on-chip stack device
* Mass Storage
	* Full-size SD Card slot (SPI)
	* Micro SD slot (SPI)
* Sensors
	* Real Time Clock (I2C)
	* Temperature/Humidity/Pressure (I2C)
* Misc
	* IDE interface (Atmega, see http://sbc.rictor.org/io/IDE.html)
	* Raspberry Pi HAT adaptor (e.g. for the SenseHAT)
	* SPI to ISA bridge (turns microATX board into full-size ATX)

## Typical Back Panel layout

```
                                        +---+  +---+  +---+  +---+
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
                                        |   |  |   |  |   |  |   |
                             +-----+    |   |  |   |  |   |  |   |
                             |_____|    |   |  |   |  |   |  |   |
+----+  +-------+            |-----|    |   |  |   |  |   |  |   |
|/--\|  | ..... |   +---+    |_____|    |   |  |   |  |   |  |   |
||__||  | ::::: |   | O |    |-----|    |   |  |   |  |   |  |   |
+----+--+-------+---+---+----+-----+----+---+--+---+--+---+--+---+
 Power   Monitor   Line-Out  2x USB A   SlotA  SlotB  SlotC  SlotD
 USB B    VGA     3.5mm TRS
```

## Pins which don't need an expansion card

* VGA_HSYNC (PB4)
* VGA_VSYNC (PB5)
* VGA_R (PF1)
* VGA_G (PB7)
* VGA_B (PD3)
* AUDIO_LEFT (PE4)
* AUDIO_RIGHT (PE5)

They are routed to the expansion connector anyway, in case you want to fit a
VGA/Audio to HDMI adaptor or something, or a VGA buffer amplifier.

## Physical Layout

When the expansion board is connected to the MCU board, they form a standard ATX
sized motherboard. The mounting holes are in the ATX standard position, and
the four expansion slots are in the same position as the equivalent ISA card
slot would be - you need to solder a 0.1" socket to the edge of a 1.6mm PCB
(putting the pins on either side of the PCB).

## CPU Boards

STM32 boards have the widest range.

### Nucleo

* F4DISCO
* H745I-DISCO

