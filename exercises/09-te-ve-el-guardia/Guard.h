#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"

UCLASS()
class AGuard : public ACharacter
{
	GENERATED_BODY()

public:
	/** true si el guardia puede ver a Target. */
	bool CanSee(const AActor* Target) const;

	/** Distancia máxima de visión (cm). */
	UPROPERTY(EditAnywhere, Category = "Vision")
	float SightRadius = 800.f;

	/** Medio ángulo del cono de visión (grados): ve hasta 45° a cada lado de su forward. */
	UPROPERTY(EditAnywhere, Category = "Vision")
	float HalfAngleDegrees = 45.f;
};
