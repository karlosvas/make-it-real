#include "Guard.h"

AGuard::AGuard()
{
	PrimaryActorTick.bCanEverTick = true;
	GetCharacterMovement()->MaxWalkSpeed = 300.f;
}

void AGuard::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	// TODO:
	//  1. Anda hacia Waypoints[CurrentIndex].
	//  2. Cuando estés a menos de AcceptanceRadius, pasa al siguiente.
	//  3. Después del último, vuelve al primero.
	//
	// ¡Cuidado! Puede que no haya ningún waypoint.
}
