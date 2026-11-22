// Conexión entre el ejercicio y el mundo simulado (el usuario no lo ve).
#include "Mir/Glue.h"
#include "Door.h"
#include "Monigote.h"

AActor* MirCreateActor(int32 ClassId)
{
	switch (ClassId)
	{
	case MirClass_Monigote: return new AMonigote();
	case MirClass_Door: return new ADoor();
	default: return new AActor();
	}
}

void MirWire(int32 Slot, int32 A, int32 B)
{
	if (Slot == 1) { MirGet<AMonigote>(A)->TargetDoor = MirGet<ADoor>(B); }
}

int32 MirCall(int32, int32, int32) { return 0; }
