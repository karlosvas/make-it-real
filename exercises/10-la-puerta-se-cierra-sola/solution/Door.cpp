#include "Door.h"

void ADoor::Open()
{
	bIsOpen = true;
	UE_LOG(LogTemp, Log, TEXT("Puerta abierta"));

	// Si CloseTimerHandle ya tenía un temporizador en marcha, SetTimer lo reemplaza:
	// la cuenta atrás vuelve a empezar desde AutoCloseDelay.
	GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::Close, AutoCloseDelay, false);
}

void ADoor::Close()
{
	bIsOpen = false;
	UE_LOG(LogTemp, Log, TEXT("Puerta cerrada"));
}
