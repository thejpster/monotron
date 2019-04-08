# Meson Build file for the I/O Controller Firmware for Monotron.
#
# See README.md.
#
# @author Jonathan Pallant <github@thejpster.org.uk>
# @copyright 2019 Jonathan Pallant
# @licence MIT or Apache 2.0 at your option.

project('io_controller', 'c', default_options : ['buildtype=minsize', 'b_lto=true', 'c_std=c11'])

add_global_arguments('-DUART_RX0_BUFFER_SIZE=32', language : 'c')
add_global_arguments('-DUART_TX0_BUFFER_SIZE=32', language : 'c')

objcopy = find_program('avr-objcopy')

third_party_inc = include_directories('third_party')

if meson.is_cross_build()
uart_lib = files('third_party/avr-uart/uart.c')
else
uart_lib = files('src/fake_uart.c')
endif

elf = executable('image', 'src/main.c', uart_lib, include_directories: third_party_inc)

if meson.is_cross_build()
custom_target('image.hex',
  input : elf,
  output : 'image.hex',
  build_by_default: true,
  command : [objcopy, '-O', 'ihex', '-R', '.eeprom', '@INPUT@', '@OUTPUT@', ],
)
run_target('size', command : ['avr-size', '-x', elf])
endif

