# Sin llave no entras

Ahora hay una **llave** en el nivel. Cuando el monigote pasa por encima, el mundo llama automáticamente a `AMonigote::OnKeyPickedUp`.

La puerta solo debe abrirse si el monigote:

1. está **delante** (como en el ejercicio 3), **y**
2. **tiene la llave**.

Esta vez tienes **dos archivos** que editar (mira las pestañas del editor):

- **Monigote.cpp**: en `OnKeyPickedUp`, apunta que ya tienes la llave (`bHasKey`).
- **Door.cpp**: en `CanOpen`, usa `Who->HasKey()`.

## Estado en los actores

Los actores guardan su estado en **variables miembro** como `bHasKey`. En Unreal, los booleanos empiezan por `b` por convención.

> `bHasKey` es `private`: desde la puerta no puedes leerlo directamente, por eso existe `HasKey()`. Es un patrón muy común en Unreal.
