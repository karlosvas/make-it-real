#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "Engine/TargetPoint.h"

UCLASS()
class AGuard : public ACharacter
{
	GENERATED_BODY()

public:
	AGuard();

	virtual void Tick(float DeltaSeconds) override;

	/** Los puntos de la patrulla, en orden (ya vienen rellenos). */
	UPROPERTY(EditAnywhere, Category = "Patrulla")
	TArray<ATargetPoint*> Waypoints;

	/** A qué distancia (cm) se considera que ha llegado a un punto. */
	UPROPERTY(EditAnywhere, Category = "Patrulla")
	float AcceptanceRadius = 50.f;

	/** Índice del waypoint al que va ahora. */
	int32 CurrentIndex = 0;
};
