# Patrulla

El guardia tiene una lista de puntos de patrulla (`Waypoints`, actores `ATargetPoint`). Tiene que recorrerlos **en orden y en bucle**: 0 → 1 → 2 → … → último → 0 → 1 → …

Implementa `AGuard::Tick`:

1. Anda hacia `Waypoints[CurrentIndex]`.
2. Cuando estés a menos de `AcceptanceRadius`, pasa al siguiente.
3. Después del último, vuelve al primero.

El guardia **anda**: su velocidad máxima es `MaxWalkSpeed = 300 cm/s` (ya está puesta en el constructor). Muévelo con `AddMovementInput` y deja que el `CharacterMovement` aplique esa velocidad. Teletransportarlo con `SetActorLocation` (o subirle el `MaxWalkSpeed`) no vale.

## Estado entre frames

`Tick` se llama cada frame, pero las variables miembro (como `CurrentIndex`) **conservan su valor** de un frame al siguiente. Así es como un actor "recuerda" lo que estaba haciendo.

## El operador %

`a % b` es el resto de dividir `a` entre `b`. Es perfecto para dar la vuelta:

```cpp
CurrentIndex = (CurrentIndex + 1) % Waypoints.Num();   // 0, 1, 2, 0, 1, 2...
```

> En el escenario "Sin waypoints" el array está vacío: ni `Waypoints[0]` ni `% 0` (que en C++ es un crash: división entre cero).
