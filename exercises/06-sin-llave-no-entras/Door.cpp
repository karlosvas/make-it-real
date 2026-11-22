#include "Door.h"
#include "Monigote.h"

bool ADoor::CanOpen(const AMonigote* Who) const
{
	if (!Who)
	{
		return false;
	}

	const FVector DoorToWho = (Who->GetActorLocation() - GetActorLocation()).GetSafeNormal();
	const bool bInFront = FVector::DotProduct(GetActorForwardVector(), DoorToWho) > 0.0;

	// TODO: además de estar delante, Who tiene que tener la llave.
	return bInFront;
}
