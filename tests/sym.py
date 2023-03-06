import symtable
import json
# read fib.py to lines
table = symtable.symtable("fib", 'fib.py', 'exec')
for name in table.get_identifiers():
    print(name)
child = table.get_children()[0]
print(child.get_parameters())
