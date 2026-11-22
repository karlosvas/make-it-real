#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS()
class ADoor : public AActor
{
	GENERATED_BODY()

public:
	ADoor();

	virtual void Tick(float DeltaSeconds) override;

	/** Lo llama el monigote para abrir la puerta. */
	void Open() { bWantsOpen = true; }

	/** Lo llama el monigote para cerrarla. */
	void Close() { bWantsOpen = false; }

	/** La hoja de la puerta. Gírala con DoorMesh->SetRelativeRotation(...). */
	UPROPERTY(VisibleAnywhere, Category = "Puerta")
	UStaticMeshComponent* DoorMesh = nullptr;

	/** Ángulo (yaw) de la hoja cuando está abierta, en grados. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float OpenYaw = 90.f;

	/** Velocidad para FMath::FInterpTo. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float OpenSpeed = 3.f;

private:
	/** ¿Queremos la puerta abierta? */
	bool bWantsOpen = false;

	/** Yaw actual de la hoja, en grados (0 = cerrada). */
	float CurrentYaw = 0.f;
};
