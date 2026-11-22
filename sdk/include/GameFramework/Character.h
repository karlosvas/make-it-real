// APawn y ACharacter: actores que se mueven (subconjunto de la API de Unreal).
#pragma once
#include "GameFramework/Actor.h"

class UCharacterMovementComponent : public UActorComponent
{
public:
	using Super = UCharacterMovementComponent;
	/** Velocidad máxima al andar, en cm/s. */
	float MaxWalkSpeed = 400.f;
	/** Si es true, el personaje gira solo hacia donde se mueve. */
	bool bOrientRotationToMovement = true;
};

class APawn : public AActor
{
public:
	using Super = APawn;

	/**
	 * Pide moverse en la dirección WorldDirection este frame.
	 * El movimiento real lo hace el componente de movimiento (como en Unreal):
	 * velocidad = dirección * MaxWalkSpeed.
	 */
	virtual void AddMovementInput(FVector WorldDirection, float ScaleValue = 1.f, bool bForce = false);
	FVector GetVelocity() const override;
};

class ACharacter : public APawn
{
public:
	using Super = ACharacter;

	UCharacterMovementComponent* GetCharacterMovement() const { return const_cast<UCharacterMovementComponent*>(&CharacterMovement); }
	void AddMovementInput(FVector WorldDirection, float ScaleValue = 1.f, bool bForce = false) override;

private:
	UCharacterMovementComponent CharacterMovement;
};
