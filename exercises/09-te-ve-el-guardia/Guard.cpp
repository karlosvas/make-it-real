#include "Guard.h"

bool AGuard::CanSee(const AActor* Target) const
{
	// TODO: el guardia ve a Target si:
	//  1. está a SightRadius o menos, y
	//  2. el ángulo entre su forward y la dirección hacia Target es HalfAngleDegrees o menos.
	return false;
}
