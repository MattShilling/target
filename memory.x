/**
 * STM32F405RG specific memory layout.
 */

MEMORY
{
    /**
     * FLASH and RAM are mandatory memory regions.
     * Flash: See 3.3 Embedded Flash memory in STM32F405
     */

    /**
     * The Flash memory is organized as follows:
     * - A main memory block divided into 4 sectors of 16 Kbytes, 
     * - 1 sector of 64 Kbytes,
     * - 7 sectors of 128 Kbytes
     * Sector 0 stars @ 0x0800 0000.
     */
    FLASH : ORIGIN = 0x08000000, LENGTH = 64K

    /**
     * - Main internal SRAM1 (112 KB)
     * - Auxiliary internal SRAM2 (16 KB)
     * - CCM (core coupled memory) (64KB)
     * -- mapped at address 0x1000 0000
     * -- !accessible only by the CPU through the D-bus.
     * SRAM1 and SRAM2 mapped at address 0x2000 0000
     * and accessible by all AHB masters.
     */
    RAM : ORIGIN = 0x20000000, LENGTH = 112K
}