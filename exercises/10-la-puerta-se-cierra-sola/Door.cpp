#include "Door.h"

void ADoor::Open()
{
	bIsOpen = true;
	UE_LOG(LogTemp, Log, TEXT("Puerta abierta"));

	// TODO: programa un temporizador para que se llame a Close()
	// dentro de AutoCloseDelay segundos.
}

void ADoor::Close()
{
	bIsOpen = false;
	UE_LOG(LogTemp, Log, TEXT("Puerta cerrada"));
}
