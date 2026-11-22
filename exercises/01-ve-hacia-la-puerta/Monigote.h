#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"

class ADoor;

UCLASS()
class AMonigote : public ACharacter
{
	GENERATED_BODY()

public:
	AMonigote();

	virtual void Tick(float DeltaSeconds) override;

	/** La puerta a la que tiene que ir. Ya viene asignada, como si la eligieras en el panel Details. */
	UPROPERTY(EditAnywhere, Category = "Monigote")
	ADoor* TargetDoor = nullptr;
};
