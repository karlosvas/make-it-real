# La puerta se cierra sola

El monigote abre la puerta con `Open()`… y se queda abierta para siempre. Queremos una puerta automática de verdad:

1. **Se cierra sola** `AutoCloseDelay` segundos (3 s) después del **último** `Open()`. Si alguien la vuelve a abrir, la cuenta empieza de nuevo.
2. **No se cierra encima de nadie.** Si cuando se acaba la cuenta el monigote está a menos de `DoorwayRadius` (120 cm) de la puerta, no la cierres: vuelve a intentarlo cada `RetryInterval` segundos (0,5 s) hasta que el paso quede libre.

Completa en `Door.cpp`:

- `Open()`: además de abrir, pone en marcha la cuenta atrás.
- `TryAutoClose()`: lo llama el temporizador. Cierra con `Close()` (ya está hecho) o lo vuelve a intentar más tarde.
- `IsDoorwayBlocked()`: `true` si el monigote está en medio. `Monigote` ya viene asignado desde el nivel.

## Temporizadores

En vez de contar el tiempo a mano en el `Tick`, Unreal tiene el *Timer Manager*:

```cpp
GetWorldTimerManager().SetTimer(
    MiHandle,          // FTimerHandle: identifica el temporizador
    this,              // objeto sobre el que se llamará al método
    &AMiClase::Metodo, // puntero al método (sin paréntesis)
    3.0f,              // segundos
    false);            // ¿repetir en bucle?
```

- Si `MiHandle` ya tenía un temporizador en marcha, `SetTimer` lo **reemplaza** y la cuenta empieza de cero.
- `GetWorldTimerManager().ClearTimer(MiHandle)` lo cancela.
- El método al que llama un temporizador puede volver a programar un temporizador.

## Lo que se comprueba

- Si el monigote vuelve a abrir la puerta **mientras está esperando** a que se aparte alguien, la cuenta vuelve a empezar desde `AutoCloseDelay`. Cuidado si usas más de un `FTimerHandle`.
- `Close()` se llama **una sola vez**. Si usas un temporizador en bucle, páralo.
