# Ábrete con suavidad

Cuando el monigote llama a `Open()`, la puerta **salta de 0° a 90° en un solo frame**. Queda fatal.

Arregla `ADoor::Tick` para que la hoja **gire poco a poco** hasta su ángulo.

## Interpolar en el Tick

El patrón más usado en Unreal para animar un valor por código:

```cpp
Valor = FMath::FInterpTo(Valor, Objetivo, DeltaSeconds, Velocidad);
```

En cada frame acerca `Valor` al `Objetivo` un trozo proporcional a lo que falta: rápido al principio y suave al final.

Después aplica el ángulo a la hoja con:

```cpp
DoorMesh->SetRelativeRotation(FRotator(0.f, CurrentYaw, 0.f));
```

> `FRotator(Pitch, Yaw, Roll)`: el **Yaw** es el giro a izquierda y derecha.

Se comprueba que:

- no gire más de 10° en un solo frame,
- esté abierta (≈90°) a los 3 s de `Open()`,
- y vuelva a ≈0° a los 3 s de `Close()`.
