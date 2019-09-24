# print demangled symbols by default
set print asm-demangle on

# Connect to the JLink GDB server
target remote :2331

# Enable SWO
monitor SWO EnableTarget 0 0 1 0

# Set a breakpoint at main
# break main


# reset to start
monitor reset

# Load the program
load
