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

	// TODO: ahora se choca con la puerta. Haz que se pare
	// cuando esté a 1 metro (100 cm) o menos.
	AddMovementInput(ToDoor.GetSafeNormal());
}
