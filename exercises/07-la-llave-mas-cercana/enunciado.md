# La llave más cercana

Hay varias llaves repartidas por el nivel. El monigote tiene un array `Keys` con todas ellas, y quiere ir a por **la más cercana**.

Implementa `AMonigote::FindClosestKey()`:

- devuelve la llave de `Keys` **más cercana** al monigote,
- y si **no hay ninguna**, devuelve `nullptr`.

El mundo llamará a tu función y el monigote irá andando a por la llave que devuelvas.

## TArray

`TArray` es el array dinámico de Unreal (como `std::vector`):

```cpp
for (AKey* Key : Keys)
{
    // ...
}

Keys.Num();      // número de elementos
Keys[0];         // primer elemento (¡si existe!)
```

> **Cuidado con `Keys[0]`**: si el array está vacío, en Unreal el editor se cierra con
> `Array index out of bounds`. Aquí también, y lo verás en el escenario "Sin llaves".

**Truco pro:** para comparar distancias no hace falta la raíz cuadrada. `FVector::DistSquared` es más barata y da el mismo orden.
