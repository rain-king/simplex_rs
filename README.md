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
- According to [https://www.pmcalculators.com/simplex-method-calculator/](https://www.pmcalculators.com/simplex-method-calculator/?problem=('cr![.2-6-3-4*4-1572573*C--*-C-*3B4-2*CCC18]~co!.2-3B158~s!.99AA=7=8~r!.3B3-4B18~o!'Maximizar')*08,.-07.['7'~8']9≥7A≤7B-2-C17CBA987.-*_), the solution to `min_example4.txt` is correct.
- Testing with `np.optimization.linprog`, the `min_example4.txt` gives a higher target result (in a minimization problem), suggesting `linprog` is failing to reach the optimal solution with both the highs and simplex method.
- Testing maximization problems is a more trivial matter. So the `example2.txt` was done by hand and returned a correct answer in our program.

# TO-DO
1. Assert correct dimensions.
2. Determine if the solution is unbounded.
3. Add command line option to choose float precisio, which might be needed with a higher number of variables or constraints.
