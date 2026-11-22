#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS()
class ADoor : public AActor
{
	GENERATED_BODY()

public:
	ADoor()
	{
		PrimaryActorTick.bCanEverTick = true;
		DoorMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("DoorMesh"));
	}

	/** (Ya hecho) Gira la hoja hacia abierta o cerrada según bIsOpen. */
	virtual void Tick(float DeltaSeconds) override
	{
		Super::Tick(DeltaSeconds);
		CurrentYaw = FMath::FInterpConstantTo(CurrentYaw, bIsOpen ? 90.f : 0.f, DeltaSeconds, 360.f);
		DoorMesh->SetRelativeRotation(FRotator(0.f, CurrentYaw, 0.f));
	}

	/** Lo llama el monigote al empujar la puerta. */
	void Open();

	/** Cierra la puerta. */
	void Close();

	bool IsOpen() const { return bIsOpen; }

	/** Segundos que tarda en cerrarse sola. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float AutoCloseDelay = 3.f;

	UPROPERTY(VisibleAnywhere, Category = "Puerta")
	UStaticMeshComponent* DoorMesh = nullptr;

private:
	bool bIsOpen = false;
	float CurrentYaw = 0.f;

	/** Identifica el temporizador de cierre. */
	FTimerHandle CloseTimerHandle;
};
