#!/bin/bash
mop() {
    ## math runs math operator with regb and regc into rega
    ## e.g 15 + 25
    rega=$( expr $regb $1 $regc )
}
rbs() {
    ## regbset sets value to regb
    regb=$1
}
rcs() {
    ## regcset sets value to regc
    regc=$1
}
dbf() {
    ## display bffr data to screen
    echo $bffr
}
cbb() {
    ## copy bffr to regb
    regb=$bffr
}
cbc() {
    ## copy bffr to regc
    regc=$bffr
}
ccb() {
    ## copy regc to bffr
    bffr=$regc
}
crb() {
    ## copy regb to bffr
    bffr=$regb
}
cba() {
    ## copy rega to bffr
    bffr=$rega
}
cbf() {
    ## clear bffr
    bffr=''
}
pint() {
    rega=''
    regb=''
    regc=''
    bffr=''
    echo [initialized palang V2.6]
}
uib() {
    read regb
}
uic() {
    read regc
}
prt() {
    echo $1
}
ldf() {
    bffr=$( cat $1 )
}
svf() {
    echo $bffr > $1
}
cif() {
    if [ "$1" == "$2" ]; then
        rega=$3
    fi
}
ose() {
    $1
}
# pint - initializes variables
# cbf - clear bffr
# cba - copy rega to bffr
# crb - copy regb to bffr
# ccb - copy regc to bffr
# cbc - copy bffr to regc
# cbb - copy bffr to regb
# dbf - display contents of buffer to screen
# rcs - sets value or string to regc
# rbs - sets value or string to regb
# mop - performs math operation with regb operator regc
# uib - get user input to regb
# uic - get user input to regc
# prt - print text
# ldf - load file into buffer
# svf - save file from buffer
# cif - if statment use as regc = bffr set rega 165
# ose - execute shell command

# put your palang code here
pint
prt 'Demo for palang V2.6'
prt 'enter first number to add'
uib
prt 'enter second number'
uic
mop +
cba
dbf
prt 'saving to output.txt'
svf output.txt
cbf
prt 'loading file output.txt'
ldf output.txt
prt 'file contents of output.txt saved to bffr'
dbf
prt 'end of demo!'
