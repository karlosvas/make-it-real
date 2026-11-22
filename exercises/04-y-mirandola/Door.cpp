#include "Door.h"

bool ADoor::CanOpen(const AActor* Who) const
{
	if (!Who)
	{
		return false;
	}

	// Esto ya lo tienes del ejercicio 3: ¿está delante?
	const FVector DoorToWho = (Who->GetActorLocation() - GetActorLocation()).GetSafeNormal();
	const bool bInFront = FVector::DotProduct(GetActorForwardVector(), DoorToWho) > 0.0;

	// TODO: además, Who tiene que estar MIRANDO a la puerta:
	// su forward vector debe estar a 45 grados o menos de la dirección hacia la puerta.
	return bInFront;
}
