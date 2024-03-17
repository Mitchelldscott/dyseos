

_phys_dram_start = 0;
_phys_bin_start = 0x80000;

PAGE_SIZE = 64K;
PAGE_MASK = PAGE_SIZE - 1;

ENTRY(_phys_bin_start);

PHDRS {
	segment_boot_core_stack PT_LOAD FLAGS(6);
	segment_code            PT_LOAD FLAGS(5);
	segment_data            PT_LOAD FLAGS(6);
}

SECTIONS
{
	. = _phys_dram_start;

	_sbcstack = .;
	.boot_core_stack (NOLOAD) :
	{
												/*   ^             */
												/*   | stack       */
		. += _phys_bin_start;        			/*   | growth      */
												/*   | direction   */
		_ebcstack = .;
	} :segment_boot_core_stack

	ASSERT((. & PAGE_MASK) == 0, "End of boot core stack is not page aligned")

	/***********************************************************************************************
    * Code + RO Data + Global Offset Table
    ***********************************************************************************************/
    _scode = .;

	.text :
	{
		KEEP(*(.text._start))
		*(.text._init_mem)
		*(.text._park)
		*(.text._kernel_init)
		*(.text*)
	}: segment_code

	.rodata : ALIGN(8) { *(.rodata*) } :segment_code

	. = ALIGN(PAGE_SIZE);
	_ecode = .;

	/***********************************************************************************************
	* Data + BSS
	***********************************************************************************************/
	.data : {

		_sdata = .;
		*(.data*);
		_edata = .;

	} :segment_data

	/* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
	.bss (NOLOAD) : ALIGN(16)
	{
		_sbss = .;
		*(.bss*);
		. = ALIGN(16);
		_ebss = .;

	} :segment_data

}