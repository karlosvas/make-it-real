#include "Monigote.h"
#include "Key.h"

AKey* AMonigote::FindClosestKey() const
{
	AKey* Closest = nullptr;
	double BestDistSq = 0.0;

	for (AKey* Key : Keys)
	{
		if (!Key)
		{
			continue;
		}

		// Comparar distancias al cuadrado es más barato y da el mismo orden.
		const double DistSq = FVector::DistSquared(GetActorLocation(), Key->GetActorLocation());
		if (!Closest || DistSq < BestDistSq)
		{
			Closest = Key;
			BestDistSq = DistSq;
		}
	}

	return Closest;
}
