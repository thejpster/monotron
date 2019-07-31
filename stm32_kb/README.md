# Monotron Keyboard Controller

## Introduction

A PS/2 keyboard or mouse sends a synchronous signal at 10 kHz. Because
Monotron is very busy calculating pixels to send to the VGA port, it can't
handle the interrupts (and there's no SPI peripherals left to handle it in
hardware).

Instead, we use this separate little STM32F0, which controls the:

* PS/2 Keyboard port
* PS/2 Mouse port

We used to use an AVR, but avr-rust wasn't really production ready, so I
replaced it with a TSSOP-20 STM32F0 on a TSSOP to DIP breakout board.
Unfortunately this means there aren't enough pins to drive the parallel port.

It talks to the main Monotron MCU over a serial link at 19,200 bps (8-data
bits, no parity, 1 stop bit).

## Compilation

This firmware is a standard Cortex-M Rust application. Build with `cargo build --release`.

## Pinout

| Port   | Pin |  Direction     | Name          | Routed to  | Description              |
|--------|-----|----------------|------------- -|------------|--------------------------|
| A      | 9   | I/O Open-drain | KB_CLK        | J14 pin 5  | PS/2 Keyboard clock      |
| A      | 10  | I/O Open-drain | MS_CLK        | J15 pin 5  | PS/2 Mouse clock         |
| A      | 13  | I/O Open-drain | KB_DATA       | J14 pin 1  | PS/2 Keyboard data       |
| A      | 14  | I/O Open-drain | MS_DATA       | J15 pin 1  | PS/2 Mouse data          |
| A      | 3   | Input          | UART_RX       | U1 PE1     | UART receive from MCU    |
| A      | 4   | Output         | UART_TX       | U1 PE0     | UART transmit to MCU     |

Refer to [the schematic](../pcb/schematic.pdf) for more details.

## Protocol

The protocol between this keyboard processor and the Monotron main MCU uses
[`illyria`](https://github.com/thejpster/illyria). This is a simple COBS
encoded packet system, with a basic ACK/NACK mechanism.

A packet looks like this (where each `[ block ]` is a single octet):

```
[ header ] [ length ] [ payload0 ] ... [ payloadN-1 ] [ checksumHigh ] [ checksumLow ]
```

The checksum is an X25 CRC16. Packets are converted to frames before
transmission, using [COBS
framing](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing).

Each packet is either an I-Frame (containing a request, confirmation or
indication) or an S-Frame (containing an ACK or a NAK).

### I-Frames

The messages are defined in the crate 'monotron-io-protocol'.

### S-Frames

* ACK (Header 0x01, no payload)
* NACK (Header 0x02, no payload)

## Data Format

Raw scan-codes are sent in the 'chunks' they are received from the keyboard or
mouse. That is, if you release the key labelled 'A' on a UK or US-English
keyboard which uses Scancode Set 2, you will get a two-byte chunk containing
'Release' (0xF0) and 'Keycode A' (0x1E).

Reading/writing from one device (e.g. the keyboard) will temporarily disable
the other device (e.g. the mouse).

## Licence

The code is Copyright (c) Jonathan 'theJPster' Pallant 2019. It is available
under the Apache 2.0 or MIT licences, at your option.
