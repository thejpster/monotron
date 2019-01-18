# Meson Build file for the I/O Controller Firmware for Monotron.
#
# See README.md.
#
# @author Jonathan Pallant <github@thejpster.org.uk>
# @copyright 2019 Jonathan Pallant
# @licence MIT or Apache 2.0 at your option.

project('io_controller', 'c', default_options : ['buildtype=minsize', 'b_lto=true', 'c_std=c11'])

objcopy = find_program('avr-objcopy')

elf = executable('image', 'src/main.c')

custom_target('image.hex',
  input : elf,
  output : 'image.hex',
  build_by_default: true,
  command : [objcopy, '-O', 'ihex', '-R', '.eeprom', '@INPUT@', '@OUTPUT@', ],
)

run_target('size', command : ['avr-size', '-x', elf])
