// Conexión entre el ejercicio y el mundo simulado (el usuario no lo ve).
#include "Mir/Glue.h"
#include "Key.h"
#include "Monigote.h"

AActor* MirCreateActor(int32 ClassId)
{
	switch (ClassId)
	{
	case MirClass_Monigote: return new AMonigote();
	case MirClass_Key: return new AKey();
	default: return new AActor();
	}
}

void MirWire(int32 Slot, int32 A, int32 B)
{
	if (Slot == 2) { MirGet<AMonigote>(A)->Keys.Add(MirGet<AKey>(B)); }
}

int32 MirCall(int32 Function, int32 A, int32)
{
	switch (Function)
	{
	case 2: return MirHandleOf(MirGet<AMonigote>(A)->FindClosestKey());
	default: return 0;
	}
}
