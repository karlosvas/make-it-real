# Para a un metro

Tu monigote ya sabe ir hacia la puerta… pero no sabe parar y **se estampa contra ella**.

Modifica el `Tick` para que **se detenga cuando esté a 1 metro (100 cm) o menos** de la puerta.

## La distancia

La longitud de un vector se obtiene con `Size()`:

```cpp
const double Distance = ToDoor.Size();
```

o, directamente entre dos puntos:

```cpp
const double Distance = FVector::Dist(GetActorLocation(), TargetDoor->GetActorLocation());
```

## Depurar

Si quieres ver los valores, escribe en el **Output Log**:

```cpp
UE_LOG(LogTemp, Warning, TEXT("Distancia: %.1f"), Distance);
```

> Se comprueba que acaba quieto entre 85 y 110 cm de la puerta y que nunca llega a tocarla.
