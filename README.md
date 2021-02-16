# Roll the Dice Discord Bot!

Use me to simulate dice rolls by sending a message of the form `/rtd ROLL`

`ROLL` is described by the grammar:

`ROLL := N | dP | NdP | (ROLL) | ROLL op ROLL`

`op := + | - | / | *`

Where `N` is any non-negative integer, `P` is any positive integer.

Expressions are computed with a PMDAS order of operation (sorry, no exponentiation).
