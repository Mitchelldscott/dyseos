

__rpi_phys_dram_start = 0;
__rpi_phys_bin_start = 0x80000;

PAGE_SIZE = 64K;
PAGE_MASK = PAGE_SIZE - 1;

ENTRY(__rpi_phys_bin_start);

EXTERN(_start)
EXTERN(_park)

PHDRS {
	segment_boot_core_stack PT_LOAD FLAGS(6);
	segment_code            PT_LOAD FLAGS(5);
	segment_data            PT_LOAD FLAGS(6);
}

SECTIONS
{
	. = __rpi_phys_dram_start;

	.boot_core_stack (NOLOAD) :
	{
												/*   ^             */
												/*   | stack       */
		. += __rpi_phys_bin_start;        		/*   | growth      */
												/*   | direction   */
		__boot_core_stack_end_exclusive = .;
	} :segment_boot_core_stack

	ASSERT((. & PAGE_MASK) == 0, "End of boot core stack is not page aligned")

	/***********************************************************************************************
    * Code + RO Data + Global Offset Table
    ***********************************************************************************************/
    __code_start = .;

	.text :
	{
		KEEP(*(.text._start))
		*(.text._init_section)
		*(.text._park)
		*(.text*)
	}: segment_code

	.rodata : ALIGN(8) { *(.rodata*) } :segment_code

	. = ALIGN(PAGE_SIZE);
	__code_end_exclusive = .;

	/***********************************************************************************************
	* Data + BSS
	***********************************************************************************************/
	.data : { *(.data*) } :segment_data

	/* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
	.bss (NOLOAD) : ALIGN(16)
	{
		__bss_start = .;
		*(.bss*);
		. = ALIGN(16);
		__bss_end_exclusive = .;

	} :segment_data

}