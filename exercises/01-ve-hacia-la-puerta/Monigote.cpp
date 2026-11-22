#include "Monigote.h"
#include "Door.h"

AMonigote::AMonigote()
{
	PrimaryActorTick.bCanEverTick = true;
}

void AMonigote::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	// TODO: haz que el monigote ande hacia TargetDoor.
}
