#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"

UCLASS()
class AMonigote : public ACharacter
{
	GENERATED_BODY()

public:
	AMonigote() { PrimaryActorTick.bCanEverTick = true; }
};
