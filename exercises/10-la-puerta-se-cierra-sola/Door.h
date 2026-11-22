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

	/** Lo llama el monigote al empujar la puerta (también si ya estaba abierta). */
	void Open();

	/** Se llama cuando se acaba la cuenta atrás: cierra la puerta o lo vuelve a intentar más tarde. */
	void TryAutoClose();

	/** true si hay alguien en medio de la puerta (a menos de DoorwayRadius). */
	bool IsDoorwayBlocked() const;

	/** (Ya hecho) Cierra la puerta. */
	void Close()
	{
		bIsOpen = false;
		UE_LOG(LogTemp, Log, TEXT("Puerta cerrada"));
	}

	bool IsOpen() const { return bIsOpen; }

	/** Segundos que tarda en cerrarse sola desde el último Open(). */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float AutoCloseDelay = 3.f;

	/** Si hay alguien en medio, cada cuántos segundos vuelve a intentar cerrarse. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float RetryInterval = 0.5f;

	/** A menos de esta distancia (cm) de la puerta se considera que estás en medio. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	float DoorwayRadius = 120.f;

	/** (Ya asignado en el nivel) Quien se puede quedar en medio. */
	UPROPERTY(EditAnywhere, Category = "Puerta")
	AActor* Monigote = nullptr;

	UPROPERTY(VisibleAnywhere, Category = "Puerta")
	UStaticMeshComponent* DoorMesh = nullptr;

private:
	bool bIsOpen = false;
	float CurrentYaw = 0.f;

	/** Identifica el temporizador de cierre. */
	FTimerHandle CloseTimerHandle;
};
