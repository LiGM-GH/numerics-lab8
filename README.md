# README.md

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
 /  -u'' - u' = -2x + 3, x ∈ (2, 4)
-|  u(2) = -12
 \_ u(4) = -10
```
### Analytical solution:
```math
Let's integrate the whole equation:
-u' - u = -x² + 3x
-λ - 1 = 0; λ = -1;
u = C * e^(-x)
u' = C' * e^(-x) - C * e(-x)
-C' * e^(-x) = -x² + 3x
-C =
= ∫ {3 * x * e^x} dx + ∫ {-e^x * x²} dx =
= |
  | u = x
  | dv = e^x dx
  | v = e^x
  | du = dx

= ∫ {-e^x * x²} dx + 3 * (x * e^x - ∫ {e^x} dx) =
= |
  | u = x^2
  | dv = e^x dx
  | v = e^x
  | du = 2xdx
= -(x² * e^x - 2 * ∫ {e^x * x} dx) + 3 * (x - 1) * e^x =
=              |
               | u = x
               | dv = e^x dx
               | v = e^x
               | du = dx
= -(x² * e^x - 2 * (x*e^x - ∫ {e^x} dx)) + 3 * (x - 1) * e^x =
= -x² * e^x + 2 * x * e^x - 2 * e^x + 3 * (x - 1) * e^x =
= -x² * e^x + 5 * x * e^x - 5 * e^x

Then
C =
= -x² * e^x + 5 * x * e^x - 5 * e^x

Then
u = x² - 5 * x + 5
u' = 2x - 5
u'' = 2
-2 - 2 x + 5 = -2x + 3
```

## Looking for approximate answer with accuracy of ε = 0.001
