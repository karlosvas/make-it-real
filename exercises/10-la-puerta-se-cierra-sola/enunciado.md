# La puerta se cierra sola

El monigote abre la puerta con `Open()`… y se queda abierta para siempre. Queremos que **se cierre sola a los `AutoCloseDelay` segundos** (3 s).

Completa `ADoor::Open()` usando un **temporizador** de Unreal.

## Temporizadores

En vez de contar el tiempo a mano en el `Tick`, Unreal tiene el *Timer Manager*:

```cpp
GetWorldTimerManager().SetTimer(
    MiHandle,          // FTimerHandle: identifica el temporizador
    this,              // objeto sobre el que se llamará al método
    &AMiClase::Metodo, // puntero al método
    3.0f,              // segundos
    false);            // ¿repetir en bucle?
```

## El detalle importante

En el segundo escenario el monigote abre la puerta y, **2 segundos después, la vuelve a abrir**. La puerta debería cerrarse **3 s después de la última vez** (a los 5 s), no a los 3 s.

Si reutilizas el mismo `FTimerHandle`, `SetTimer` **reemplaza** el temporizador anterior y la cuenta empieza de nuevo. Es exactamente lo que queremos.
