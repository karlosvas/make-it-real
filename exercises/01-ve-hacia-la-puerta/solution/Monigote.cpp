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

	// Vector que va desde el monigote hasta la puerta.
	const FVector ToDoor = TargetDoor->GetActorLocation() - GetActorLocation();

	// Solo nos interesa la dirección: lo normalizamos (longitud 1).
	AddMovementInput(ToDoor.GetSafeNormal());
}
