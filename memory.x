MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 4K
}