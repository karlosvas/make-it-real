#include "Door.h"

void ADoor::Open()
{
	bIsOpen = true;
	UE_LOG(LogTemp, Log, TEXT("Puerta abierta"));

	// Siempre el mismo handle: si ya había una cuenta atrás (o un reintento) en marcha,
	// SetTimer la reemplaza y la cuenta vuelve a empezar desde AutoCloseDelay.
	GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, AutoCloseDelay, false);
}

void ADoor::TryAutoClose()
{
	if (IsDoorwayBlocked())
	{
		UE_LOG(LogTemp, Log, TEXT("Hay alguien en medio: espero"));
		// Se puede volver a programar desde el propio método que llama el temporizador.
		// Con el mismo handle, un Open() posterior también cancela este reintento.
		GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, RetryInterval, false);
		return;
	}
	Close();
}

bool ADoor::IsDoorwayBlocked() const
{
	return Monigote && GetHorizontalDistanceTo(Monigote) < DoorwayRadius;
}
