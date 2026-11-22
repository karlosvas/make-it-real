// Conexión entre el ejercicio y el mundo simulado (el usuario no lo ve).
#include "Mir/Glue.h"
#include "Guard.h"
#include "Monigote.h"

AActor* MirCreateActor(int32 ClassId)
{
	switch (ClassId)
	{
	case MirClass_Guard: return new AGuard();
	case MirClass_Monigote: return new AMonigote();
	default: return new AActor();
	}
}

void MirWire(int32, int32, int32) {}

int32 MirCall(int32 Function, int32 A, int32 B)
{
	switch (Function)
	{
	case 3: return MirGet<AGuard>(A)->CanSee(MirActor(B)) ? 1 : 0;
	default: return 0;
	}
}
