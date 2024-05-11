ENTRY(start)

SECTIONS {
    . = 1M;

    .text :
    {
        *(.multiboot_header)
        *(.text)
    }
}