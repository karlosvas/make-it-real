# La puerta solo se abre desde delante

El monigote se acerca a la puerta e intenta abrirla. El problema: **ahora se abre siempre**, también cuando el monigote llega por detrás.

Completa `ADoor::CanOpen` para que devuelva `true` **solo si `Who` está delante de la puerta**.

## Qué es "delante"

Todo actor tiene un *forward vector*: un vector de longitud 1 que apunta hacia donde mira. En la puerta, el lado de **delante** está marcado con una flecha verde en el suelo.

Si calculas la dirección **desde la puerta hasta el monigote**, el producto escalar con el forward vector te dice en qué lado está:

| `DotProduct(Forward, DirAlMonigote)` | Significa |
|---|---|
| mayor que 0 | está delante |
| 0 | justo al lado |
| menor que 0 | está detrás |

> En Unreal, X es delante, Y es derecha y Z es arriba. Las distancias van en centímetros.

Cuando funcione, verás al monigote cruzar desde delante y quedarse bloqueado cuando llega por detrás.
