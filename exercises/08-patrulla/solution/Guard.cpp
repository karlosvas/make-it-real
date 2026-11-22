#include "Guard.h"

AGuard::AGuard()
{
	PrimaryActorTick.bCanEverTick = true;
	GetCharacterMovement()->MaxWalkSpeed = 300.f;
}

void AGuard::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	if (Waypoints.Num() == 0)
	{
		return;
	}

	const ATargetPoint* Target = Waypoints[CurrentIndex];
	const FVector ToTarget = Target->GetActorLocation() - GetActorLocation();

	if (ToTarget.Size2D() < AcceptanceRadius)
	{
		// El operador % hace que después del último vuelva al 0.
		CurrentIndex = (CurrentIndex + 1) % Waypoints.Num();
		return;
	}

	AddMovementInput(ToTarget.GetSafeNormal2D());
}
