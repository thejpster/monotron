MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 256K
    /* Limit the ROM to the first 8 KiB of SRAM. The other 24 KiB of SRAM is
    for applications. */
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 16K
}
