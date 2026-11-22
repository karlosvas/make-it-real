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

	/** El mundo lo llama automáticamente cuando el monigote pasa por encima de una llave. */
	void OnKeyPickedUp(AKey* Key);

	bool HasKey() const { return bHasKey; }

private:
	bool bHasKey = false;
};
