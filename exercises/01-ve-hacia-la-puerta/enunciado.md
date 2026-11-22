# Ve hacia la puerta

¡Bienvenido a **Make It Real**! Aquí escribes C++ con la API de Unreal y ves el resultado en 3D.

El monigote está quieto. Tiene una propiedad `TargetDoor` que ya apunta a la puerta del nivel (como si la hubieras elegido en el panel *Details* del editor).

**Haz que en cada frame ande hacia la puerta.**

## Cómo se mueve un personaje en Unreal

`Tick(float DeltaSeconds)` se ejecuta en **cada frame** (unas 60 veces por segundo). Dentro, llama a:

```cpp
AddMovementInput(Direccion);
```

Eso le dice al componente de movimiento del personaje "quiero ir hacia allí". Él se encarga de moverlo a su velocidad (`MaxWalkSpeed`, 400 cm/s) y de girarlo hacia donde anda.

## La dirección

Si tienes dos posiciones, `B - A` es un vector que va **de A a B**. Para usarlo como dirección, normalízalo con `GetSafeNormal()`.

> Recuerda: en Unreal las distancias van en **centímetros**. X es delante, Y es derecha y Z es arriba.
