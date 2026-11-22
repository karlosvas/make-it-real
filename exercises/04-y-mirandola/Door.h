#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS()
class ADoor : public AActor
{
	GENERATED_BODY()

public:
	/**
	 * El monigote llama a esta función cuando intenta abrir la puerta.
	 * Devuelve true si Who puede abrirla.
	 */
	UFUNCTION(BlueprintCallable)
	bool CanOpen(const AActor* Who) const;
};
