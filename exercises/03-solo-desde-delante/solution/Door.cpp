#include "Door.h"

bool ADoor::CanOpen(const AActor* Who) const
{
	if (!Who)
	{
		return false;
	}

	// Dirección desde la puerta hasta el monigote.
	const FVector ToWho = (Who->GetActorLocation() - GetActorLocation()).GetSafeNormal();

	// Si el producto escalar es positivo, el monigote está en el mismo lado
	// hacia el que mira la puerta: está delante.
	const double Dot = FVector::DotProduct(GetActorForwardVector(), ToWho);
	UE_LOG(LogTemp, Log, TEXT("CanOpen: Dot = %.2f"), Dot);
	return Dot > 0.0;
}
