OUTPUT_ARCH("riscv")

ENTRY(_start)
EXTERN(abort);

MEMORY
{
    RAM : ORIGIN = 0x80000000, LENGTH = 256K
}

SECTIONS
{
    .reset :
    {
        KEEP(*(.reset.boot));
    } > RAM

    .text :
    {
        *(.text. text.*);
    } > RAM

    .rodata :
    {
        *(.rdata .rodata. rodata.*);
    } > RAM
}