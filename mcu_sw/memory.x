MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00080000
  CAN        : ORIGIN = 0x20000000, LENGTH = 64K
  RAM (xrw)  : ORIGIN = 0x20010000, LENGTH = 192K-64K
}

SECTIONS {
  .can (NOLOAD) :
  {
    *(.can .can.*);
  } > CAN
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);