# target extended-remote :3333

# print demangled symbols
set print asm-demangle on

set confirm off

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

set mem inaccessible-by-default off
set architecture riscv:rv32
set remotetimeout unlimited
monitor reset halt
load
continue
