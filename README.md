# RISCV-VM
A minimal VM, targeting the RISC-V architecture, written in rust.
The development flow I've been following on this project is Test Driven Development.

### Currently working
- Base RISC-V operations
- M extension operations
- Very simple register and memory viewer

### To Do
- MMU support (WIP)
- Per page view for memory (alternatively dynamically loading the memory), as the current view makes the program crawl with only 2MB of RAM.
- Better instruction flow control:
    - Pause execution
    - Execution rate
    - Step forward
    - Step backward?
- Implement atomic functions
- Simple peripherals:
    - Serial IO
    - Simple screen
- Boot into an OS