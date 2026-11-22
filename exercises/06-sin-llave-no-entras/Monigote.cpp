#include "Monigote.h"
#include "Key.h"

void AMonigote::OnKeyPickedUp(AKey* Key)
{
	UE_LOG(LogTemp, Log, TEXT("He cogido %s"), *Key->GetName());

	// TODO: apunta que ya tienes la llave.
}
