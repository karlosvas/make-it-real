#include "Door.h"
#include "Monigote.h"

bool ADoor::CanOpen(const AMonigote* Who) const
{
	if (!Who || !Who->HasKey())
	{
		return false;
	}

	const FVector DoorToWho = (Who->GetActorLocation() - GetActorLocation()).GetSafeNormal();
	return FVector::DotProduct(GetActorForwardVector(), DoorToWho) > 0.0;
}
