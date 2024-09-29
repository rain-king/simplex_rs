#!/usr/bin/python

import numpy as np
from scipy.optimize import linprog

def lp_input():
	print("Enter the c vector and press Enter again:")
	c = list(map(lambda x: float(x), input().split(" ")))
	input()

	A = list()
	print("Write the INEQUALITY rows for the A matrix line by line with values separated by spaces")
	print("and press Enter again:")
	line = str()
	while True:
		line = input()
		if line == "":
			break
		numbers = line.split(" ")

		if numbers != list(""):
			if len(numbers) != len(c):
				print("The length of an A row should be the length of c.")
				return

			row = list(map(lambda x: float(x), numbers))
			A.append(row)
			A_inconsistent = len(A) > 1 and len(A[len(A) - 1]) != len(A[len(A) - 2])
			if A_inconsistent:
				print("Err: All rows should have equal size.")
				return

	# input b
	print("Enter the b vector INEQUALITY values, and press Enter again:")
	line = input()
	input()
	if line == "":
		b = list()
	else:
		numbers = line.split(" ")
		b = list(map(lambda x: float(x), numbers))
		if len(b) != len(A):
			print("b_eq should have a length equal to the number of rows in A.")
			return

	# input A_eq
	A_eq = list()
	print("Write the EQUALITY rows for the A matrix line by line with values separated by spaces")
	print("and press Enter again:")
	while True:
		line = input()
		if line == "":
			break
		numbers = line.split(" ")

		if len(numbers) != len(c):
			print("The length of an A_eq row should be the length of c.")
			return

		row = list(map(lambda x: float(x), numbers))
		A_eq.append(row)
		A_eq_inconsistent = len(A_eq) > 1 and len(A_eq[len(A_eq) - 1]) != len(A_eq[len(A_eq) - 2])
		if A_eq_inconsistent:
			print("Err: All rows should have equal size.")
			return

	# input b_eq
	print("Enter the b vector EQUALITY values and press Enter again: ")
	line = input()
	input()
	if line == "":
		b_eq = list()
	else:
		numbers = line.split(" ")
		b_eq = list(map(lambda x: float(x), numbers))
		if len(b_eq) != len(A_eq):
			print("b_eq should have a length equal to the number of rows in A_eq.")
			return

	x_bounds = [(0,None) for x in c]

	print()
	
	return c, A, b, A_eq, b_eq, x_bounds

def main():
	c, A, b, A_eq, b_eq, x_bounds = lp_input()
	
	if A_eq == list():
		sol = linprog(c, A_ub=A, b_ub=b, bounds=x_bounds, method='highs')
	elif A == list():
		sol = linprog(c, A_eq=A_eq, b_eq=b_eq, bounds=x_bounds, method='highs')
	else:
		sol = linprog(c, A_ub=A, b_ub=b, A_eq=A_eq, b_eq=b_eq, bounds=x_bounds, method='highs', options={'disp': True, 'maxiter': 1000})

	print('Costo mínimo:', sol.fun)
	print('Solución óptima:', sol.x)

main()
