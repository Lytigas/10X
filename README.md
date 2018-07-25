# The 10X Programming Language

10X is a language that seeks to turn brainfuck into a usable language for high
performance, parallel computing using tensors. It is designed for use only by
true 10X programmers, and thus does not hesitate to expose some complexity.

## Motivation

In a ploy shamelessly stolen from [rockstar](https://github.com/dylanbeattie/rockstar):

* If we make 10X a real (and completely pointless) programming
language, then recruiters and hiring managers won't be able to talk about
'10X programmers' any more.
* 10 dimensional tensors.
* Stickers saying 'CERTIFIED 10X PROGRAMMER'?

## Specification

Execution takes place on an infinite, 10d tensor of bytes.

10X lang approaches parallelism by duplicating brainfuck-like cursors. Execution starts with the root cursor and proceeds down the cursor heirarchy. Cursors at equal layers execute in the order they were created.

When a cursor is created, it's instruction pointer is placed after the instruction that created it. It is then placed in the execution heirarchy as normal. Other cursors may execute, before the new one.

If multiple cursors attempt to occupy the same cell at any point, those lower in the execution order are killed.

Instruction set:
```
M[0-9] Move cursor in the positive direction on the given axis
m[0-9] Move cursor in the negative direction on the given axis
X[0-9] Duplicate cursor in the positive direction on the given axis
x[0-9] Duplicate cursor in the negative direction on the given axis
u Undoes the last duplication, killing all child cursors recursively
k the cursor kills itself and its child cursors
o place a cursor-local portal at the current cell. Portals are inhereted to duplicated cursors but can be overwritten. Portals co-exist with data.
j jump to the portal if the current cell is 0
+ Increment the current cells
- Decrement the current cells
| Do nothing
: Read input cell by cell at the location of each cursor. Cursors at lower positions are selected first, with lower axis having higher precedence.
. Read one input into all cells
, Output the bytes at each cell, using the same ordering as ":"
[ if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command
] if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command
```
