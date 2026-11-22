#include "Door.h"

void ADoor::Open()
{
	bIsOpen = true;
	UE_LOG(LogTemp, Log, TEXT("Puerta abierta"));

	// TODO: dentro de AutoCloseDelay segundos se tiene que llamar a TryAutoClose().
}

void ADoor::TryAutoClose()
{
	// TODO: si el paso está libre, Close().
	// Si hay alguien en medio, vuelve a intentarlo dentro de RetryInterval segundos.
}

bool ADoor::IsDoorwayBlocked() const
{
	// TODO: ¿está el Monigote a menos de DoorwayRadius de la puerta?
	return false;
}
