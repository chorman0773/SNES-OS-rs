INCLUDE wc65c816-snesdev-vectors.ld
SECTION {
    .pram.bios ALIGN(4096){
        . = .;
        __reset_bios_startup = .;
    }
}