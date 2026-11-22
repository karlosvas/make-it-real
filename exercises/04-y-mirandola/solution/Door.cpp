#include "Door.h"

bool ADoor::CanOpen(const AActor* Who) const
{
	if (!Who)
	{
		return false;
	}

	// 1) ¿Está delante de la puerta?
	const FVector DoorToWho = (Who->GetActorLocation() - GetActorLocation()).GetSafeNormal();
	const bool bInFront = FVector::DotProduct(GetActorForwardVector(), DoorToWho) > 0.0;

	// 2) ¿Está mirando hacia la puerta? Comparamos hacia dónde mira el monigote
	//    con la dirección que va del monigote a la puerta (la contraria a DoorToWho).
	const FVector WhoToDoor = -DoorToWho;
	const double CosAngle = FVector::DotProduct(Who->GetActorForwardVector(), WhoToDoor);

	// Con vectores unitarios, el producto escalar es el coseno del ángulo entre ellos.
	// Ángulo <= 45°  <=>  coseno >= cos(45°)
	const bool bLooking = CosAngle >= FMath::Cos(FMath::DegreesToRadians(45.0));

	return bInFront && bLooking;
}
