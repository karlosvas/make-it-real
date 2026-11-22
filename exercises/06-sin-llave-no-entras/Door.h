#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

class AMonigote;

UCLASS()
class ADoor : public AActor
{
	GENERATED_BODY()

public:
	/** Devuelve true si Who puede abrir la puerta. */
	UFUNCTION(BlueprintCallable)
	bool CanOpen(const AMonigote* Who) const;
};
