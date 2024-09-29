# What it does
- This program solves linear programming problems in exactly the following form:
Maximizing or minimizing $Z(x) = c\dot x$ subject to the constraints $$Ax \leq b$$ and $$A_{eq} x = b_{eq}$$ with $x \geq 0$.
Where either the inequalities or the equalities constraints can be omitted. $b$ is not limited to be non-negative, as such, one can solve
linear programming problems with $\geq$ constraints by multiplying the inequality by $-1$ (and so is done in min_example4.txt).

# How to read the from a file
You can parse a file `input.txt` with the following format using redirection `program <input.txt`
```
[input.txt contents]
<c values separated by spaces>

<A row 1 values separated by spaces>
<A row 2>
...
<A last row>

<b values separated by spaces>

<A_eq row 1>
...
<A_eq last row>

<b_eq values separated by spaces>
```
Notice the newline characters. Since when running the program interactively, the user is prompted to press Return again after every data input, except the last input.

# Efficacy tests
- With `np.optimization.linprog` (with both `highs` and `simplex` methods),
the `min_example4.txt` on our program gives the same target optimal value and the same optimal solution.
(Try with scipy installed, `./minsimplex.sh < test_example4.txt`.)
- In the case of unfeasible problems, we detect if the minimization of `W` in phase one gives a positive target.
We use `scipy/unfeasible_example4.txt` for scipy, and `examples/unbfeasible_example4.txt` for our program in testing.
- Maximization problems are a more trivial matter. So the `example2.txt` was done by hand and returned a correct answer in our program.

# TO-DO
1. Assert correct dimensions.
2. Determine if the solution is unbounded.
3. Add command line option to choose float precision, which might be needed with a higher number of variables or constraints.
4. Check if it works with only equalities. I have not found a problem with only equalities and a known solution because I haven't searched for one.
