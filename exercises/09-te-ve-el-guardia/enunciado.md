# ¿Te ve el guardia?

El guardia vigila con un **cono de visión**: ve hasta `SightRadius` (800 cm) de distancia y `HalfAngleDegrees` (45°) a cada lado de hacia donde mira.

Implementa `AGuard::CanSee(Target)`, que devuelve `true` si:

1. `Target` está a **`SightRadius` o menos**, **y**
2. el ángulo entre el *forward* del guardia y la dirección hacia `Target` es **`HalfAngleDegrees` o menos**.

El mundo llama a `CanSee` **en cada frame** mientras el monigote se mueve. El cono del visor se pone **rojo** cuando tu función dice que lo ve.

## Pista de rendimiento

`CanSee` se llama muchísimo en un juego real (cada guardia, cada frame). Dos trucos típicos:

- Compara **distancias al cuadrado**: `ToTarget.SizeSquared() > FMath::Square(SightRadius)`.
- Compara **cosenos** en vez de ángulos: así no necesitas `Acos`.

> Se comprueba en todos los frames, menos en los que el monigote está justo en el borde del cono (±1,5° o ±8 cm).
