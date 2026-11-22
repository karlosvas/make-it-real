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
	if (Slot == 4) { MirGet<ADoor>(A)->Monigote = MirActor(B); }
}

int32 MirCall(int32 Function, int32 A, int32)
{
	switch (Function)
	{
	case 4: MirGet<ADoor>(A)->Open(); return 1;
	default: return 0;
	}
}
