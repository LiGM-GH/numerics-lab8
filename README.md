# README.md
\
## Task 8.1

+---+---+-----+---+----------+------+-----+
| a | b |  p  | q |   f(x)   |  Ua  | Ub  |
+---+---+-----+---+----------+------+-----+
| 2 | 4 | -1  | 0 | -2x + 3  | -12  |-10  |
+---+---+-----+---+----------+------+-----+

Find solituon for
```math
  _
 /  -u'' + pu' + qu = f(x), x ∈ (a, b)
-|  u(a) = Ua,
 \_ u(b) = Ub.

My problem:
  _
 /  -u'' - u' = 4x + 2, x ∈ (0, 3)
-|  u(2) = 2
 \_ u(4) = 0
```
### Analytical solution:
```math
Ackshually, the solution is:
u = 2 * e^3 / (e^3 - 1) * e^(-x) - 2 / (e^3 - 1) - 2x² + 6x
```

## Looking for approximate answer with accuracy of ε = 0.001

What if we look at this:
```math
-u'' - u' = -2x + 3, x ∈ (2, 4)
```
as
```math
-(u'*e^x)' = (-2x + 3)*e^x, where k(x) = e^x
```
