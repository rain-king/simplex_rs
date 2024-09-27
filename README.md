# What it does, and what does work
In my limited testing, it works for some maximization linear programming problems given in the following form.
Maximizing $Z(x) = c\dot x$ subject to the constraints $Ax \leq b$ with $x \geq 0$ and $b \geq 0$.

# How to read the from a file
You can parse a file `input.txt` with the following format using redirection `program <input.txt`
```
[input.txt contents]
<c vector element separated by spaces>

<A row 1 values separated by spaces>
<A row 2>
...
<A last row>

<b values separated by spaces>

```
Notice the newline characters. 
