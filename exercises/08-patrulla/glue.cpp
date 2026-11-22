// Conexión entre el ejercicio y el mundo simulado (el usuario no lo ve).
#include "Mir/Glue.h"
#include "Guard.h"

AActor* MirCreateActor(int32 ClassId)
{
	switch (ClassId)
	{
	case MirClass_Guard: return new AGuard();
	case MirClass_TargetPoint: return new ATargetPoint();
	default: return new AActor();
	}
}

void MirWire(int32 Slot, int32 A, int32 B)
{
	if (Slot == 3) { MirGet<AGuard>(A)->Waypoints.Add(MirGet<ATargetPoint>(B)); }
}

int32 MirCall(int32, int32, int32) { return 0; }
