# README.md
## Task 8.1

|  a | b |  p  | q |   f(x)   |  Ua  | Ub  |
| -- | - | --- | - | -------- | ---- | ----|
|  2 | 4 | -1  | 0 | -2x + 3  | -12  |-10  |

Find solituon for
```
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
```
Ackshually, the solution is:
u = 2 * e³ / (e³ - 1) * e^(-x) - 2 / (e^3 - 1) - 2x² + 6x
```

### Looking for approximate answer with accuracy of ε = 0.001

What if we look at this:
```math
-u'' - u' = -2x + 3, x ∈ (2, 4)
```
as
```math
-(u'*e^x)' = (-2x + 3)*e^x, where k(x) = e^x
```

## Task 8.2

| a | b | k(x)       | q(x) | f(x)    | Ua | Ub |
| - | - | ---------- | ---- | ------- | -- | -- |
| 0 | 3 | 7.5 - 0.5x | 3    | 2x³ + 1 | 1  | 6  |

u(b) + 2k(b)*u'(b)=3.2
u(a) = 1.2
u[N] + 2k(b) * (u[N] - u[N - 1]) / h = 3.2
u[N] * (1 + 2k(b) / h) + u[N-1] * (-2k(b) / h) = 3.2

-(ku')' + 3u = 2x³ + 1
-(7.5u' - 0.5xu')' + 3u = 2x³ + 1
-7.5u'' + 0.5u''x + 0.5u' + 3u = 2x³ + 1
u''(-7.5 + 0.5x) +0.5u' + 3u = 2x³ + 1
