import sys
import os
code = ""
with open(sys.argv[1],'r') as file:
    code = file.read()
splitCode = code.split('\n')
i = 0
while i < len(splitCode):
  splitLine = splitCode[i].strip().split(' ')
  opcode = splitLine[0].lower()
  if opcode == "prt":
    print((' '.join(splitLine[1:])).strip('"'))
  elif opcode == "dmp":
    print(rega, regb, regc, regd, rege, bffr)
  elif opcode == "pint":
    rega = 0
    regb = 0
    regc = 0 
    regd = ""
    rege = ""
    bffr = ""
  elif opcode == "jmp":
    i = int(splitLine[1])
    continue
  elif opcode == "cbf":
    bffr = ""
  elif opcode == "cba":
    bffr = bffr + str(rega)
  elif opcode == "crb":
    bffr = bffr + str(regb)
  elif opcode == "ccb":
    bffr = bffr + str(regc)
  elif opcode == "cbd":
    regd = bffr
  elif opcode == "cbe":
    rege = bffr
  elif opcode == "dbf":
    print(bffr)
  elif opcode == "rcs":
    regc = int(splitLine[1])
  elif opcode == "rbs":
    regb = int(splitLine[1])
  elif opcode == "ldf":
    with open(splitLine[1],'r') as file:
      bffr = file.read()
  elif opcode == "ose":
    snake_case_supremacy = ' '
    splitLine = splitLine[1:]
    notSplitLine = snake_case_supremacy.join(splitLine)
    os.system(notSplitLine)
  elif opcode == "icm":
    if regb == regc:
      i = int(splitLine[1])
  elif opcode == "scm":
    if regd == rege:
      i = int(splitLine[1])
  elif opcode == "ctd":
    regd = bffr
  elif opcode == "cte":
    rege = bffr
  elif opcode == "rtd":
    regd = (' '.join(splitLine[1:])).strip('"')
  elif opcode == "rte":
    rege = (' '.join(splitLine[1:])).strip('"')
  elif opcode == "stb":
    bffr = (' '.join(splitLine[1:])).strip('"')
  elif opcode == "ltd":
    with open(splitLine[1],'r') as file:
      regd = file.read()
  elif opcode == "lte":
    with open(splitLine[1],'r') as file:
      rege = file.read()
  elif opcode == "isw":
    swp_tmp = regb
    swp_tmp2 = regc
    regb = swp_tmp2
    regc = swp_tmp
  elif opcode == "ssw":
    swp_tmp = regd
    swp_tmp2 = rege
    regd = swp_tmp2
    rege = swp_tmp
  elif opcode == "add":
    rega = regb + regc
  elif opcode == "sub":
    rega = regb - regc
  elif opcode == "mul":
    rega = regb * regc
  elif opcode == "div":
    rega = regb / regc
  elif opcode == "inc":
    regb += 1
  elif opcode == "dec":
    regb -= 1
  elif opcode == "isa":
    if rega == 0:
      i = int(splitLine[1])
  elif opcode == "isb":
    if regb == 0:
      i = int(splitLine[1])
  elif opcode == "isc":
    if regc == 0:
      i = int(splitLine[1])
  elif opcode == "uib":
    regb = int(input())
  elif opcode == "uic":
    regc = int(input())
  elif opcode == "uid":
    regd = input()
  elif opcode == "uie":
    rege = input()
  elif opcode == "dlf":
    if os.path.exists(splitLine[1]):
      os.remove(splitLine[1])
  elif opcode == "srb":
    f = open(splitLine[1], "w")
    f.write(regb)
    f.close()
  elif opcode == "sra":
    f = open(splitLine[1], "w")
    f.write(rega)
    f.close()
  elif opcode == "src":
    f = open(splitLine[1], "w")
    f.write(regc)
    f.close()
  elif opcode == "ibm":
    if regb == splitLine[1]:
        i = int(splitLine[2])
  elif opcode == "icm":
    if regc == splitLine[1]:
        i = int(splitLine[2])
  elif opcode == "idm":
    if regd == splitLine[1]:
        i = int(splitLine[2])
  elif opcode == "iem":
    if rege == splitLine[1]:
        i = int(splitLine[2])
  i += 1
