#include "Monigote.h"
#include "Door.h"

AMonigote::AMonigote()
{
	PrimaryActorTick.bCanEverTick = true;
}

void AMonigote::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	if (!TargetDoor)
	{
		return;
	}

	const FVector ToDoor = TargetDoor->GetActorLocation() - GetActorLocation();
	const double Distance = ToDoor.Size();

	// Ya hemos llegado: no pedimos más movimiento.
	if (Distance <= 100.0)
	{
		return;
	}

	AddMovementInput(ToDoor.GetSafeNormal());
}
