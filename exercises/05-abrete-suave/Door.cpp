#include "Door.h"

ADoor::ADoor()
{
	PrimaryActorTick.bCanEverTick = true;
	DoorMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("DoorMesh"));
}

void ADoor::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	const float TargetYaw = bWantsOpen ? OpenYaw : 0.f;

	// TODO: ahora la puerta "salta" de golpe a su ángulo final.
	// Haz que CurrentYaw se acerque poco a poco a TargetYaw con FMath::FInterpTo.
	CurrentYaw = TargetYaw;

	DoorMesh->SetRelativeRotation(FRotator(0.f, CurrentYaw, 0.f));
}
