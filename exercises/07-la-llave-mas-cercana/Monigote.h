#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"

class AKey;

UCLASS()
class AMonigote : public ACharacter
{
	GENERATED_BODY()

public:
	AMonigote() { PrimaryActorTick.bCanEverTick = true; }

	/** Devuelve la llave de Keys más cercana al monigote, o nullptr si no hay ninguna. */
	AKey* FindClosestKey() const;

	/** Todas las llaves del nivel (ya vienen rellenas). */
	UPROPERTY(EditAnywhere, Category = "Monigote")
	TArray<AKey*> Keys;
};
