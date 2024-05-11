.section .multiboot_header
header_start:
    .4byte 0xe85250d6                   // magic number (multiboot 2)
    .4byte 0                            // architecture 0 (x86 protected mode)
    .4byte header_end - header_start    // header length

    //checksum
    .4byte 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    
    //optional multiboot tags

    //end tag
    .2byte 0
    .2byte 0
    .4byte 8
header_end: