i use .vbl for the file extension but its not really needed
to run it make sure the interpter is in the same location and run 'python3 vbl.py file.vbl" and replace file.vbl with the program you wanna run

tips:
you can use rst anywhere in the program to reset all registers and the buffer

commands:
rst - resets all internal buffers to initial state, doesn't accept arguments
dmp - display contents of all registers and bffer
clr <opt\\reg\\id> - clear buffer if no argument is given, otherwise clear the register with the given ID
cr<id> - copy the contents of register <id> to buffer (register IDs are a single letter from a to Z, any ASCII letter)
cb<id> - copy the contents of buffer to register <ID> (register IDs are a single letter from a to Z, any ASCII letter)
dsp <opt\\reg\\id> - display contents of buffer if no argument is given, otherwise display the contents of register \_id\* to screen
st<id> <value> - sets the value of register _id_ to the given value
shl <command> - execute shell command - example:"whoami"

cmp <reg1> <reg2> <jumpto> - compares \_reg1* to \_reg2* and if they are equal, run `jmp <jumpto>`
lbl <label_name> - creates a label bound to the given name(useful for jumping to lables)

jmp <jumpto> - jumps to a specified line or to the lable with the given name - example:"jmp 20"/"jmp mainloop" (Allows relative line numbers in the format of -n to jump n lines back or +n to jump n lines forward)

dlf <file> - deletes a specified file - example:"dlf file.txt"

ld<id> <file> - loads the contents of a file given as the argument to register <ID>
sr<id> <file> - saves the contents of register <ID> to a file given as the argument
si<id> - get string input into register <ID>.
ii<id> - get integer input into register <ID>.
fi<id> - get float input into register <ID>.

swp <reg1> <reg2> - swaps the contents of registers arg1 and arg2
add <reg1> <reg2> - adds the contents of register arg1 and the contents of arg2 and saves the result into reg1
sub <reg1> <reg2> - subtracts the contents of register arg1 from the contents of register arg2
mul <reg1> <reg2> - multiplies the contents of register arg1 and contents of register arg2
div <reg1> <reg2> - divides the contents of <ID given in arg1> by the contents of register <ID given in arg2>
rem <reg1> <reg2> - gets modulus of reg1 over reg2 and sets reg1 to the result
inc <reg1> - decrements the value of a register given as argument by 1
dec <reg1> - decrements the value of a register given as argument by 1

rg<type> <register> <length(string)/range_start> <range_end> - type can be one of: `s` for a random string, `i` for a random integer(requires two arguments: a range start and end) and `f` for a random f64, syntax identical to random integers

is<id> <jumpto> - jumps to line given as argument if register <id> is equal to 0
