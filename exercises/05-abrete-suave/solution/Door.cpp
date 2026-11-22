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

	// Cada frame avanza un trocito (proporcional a lo que falta): rápido al principio y suave al final.
	CurrentYaw = FMath::FInterpTo(CurrentYaw, TargetYaw, DeltaSeconds, OpenSpeed);

	DoorMesh->SetRelativeRotation(FRotator(0.f, CurrentYaw, 0.f));
}
