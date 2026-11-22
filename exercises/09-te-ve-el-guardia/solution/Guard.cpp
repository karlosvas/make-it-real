#include "Guard.h"

bool AGuard::CanSee(const AActor* Target) const
{
	if (!Target)
	{
		return false;
	}

	const FVector ToTarget = Target->GetActorLocation() - GetActorLocation();

	// 1) Distancia (comparando al cuadrado nos ahorramos la raíz).
	if (ToTarget.SizeSquared() > FMath::Square(SightRadius))
	{
		return false;
	}

	// 2) Ángulo: coseno entre el forward y la dirección al objetivo.
	const double CosAngle = FVector::DotProduct(GetActorForwardVector(), ToTarget.GetSafeNormal());
	return CosAngle >= FMath::Cos(FMath::DegreesToRadians(HalfAngleDegrees));
}
