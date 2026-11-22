# Y además, mirándola

Los diseñadores quieren que el monigote solo pueda abrir la puerta si está **delante** *y además* la está **mirando**. Nada de abrir puertas de espaldas.

Completa `ADoor::CanOpen` para que devuelva `true` solo si:

1. `Who` está **delante** de la puerta (lo del ejercicio 3), **y**
2. el *forward* de `Who` apunta hacia la puerta con un ángulo de **45° o menos**.

## El producto escalar como ángulo

Si `A` y `B` son vectores **unitarios** (longitud 1):

```cpp
FVector::DotProduct(A, B) == cos(ángulo entre A y B)
```

| Ángulo | Coseno |
|---|---|
| 0° (misma dirección) | 1 |
| 45° | 0.707 |
| 90° | 0 |
| 180° (opuestos) | -1 |

Así que "ángulo ≤ 45°" es lo mismo que "producto escalar ≥ cos(45°)". Y no hace falta calcular ningún `Acos`.

> `FMath::Cos` trabaja en **radianes**: usa `FMath::DegreesToRadians(45.0)`.
