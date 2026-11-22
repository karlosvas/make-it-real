// Conexión entre el ejercicio y el mundo simulado (el usuario no lo ve).
#include "Mir/Glue.h"
#include "Door.h"
#include "Key.h"
#include "Monigote.h"

AActor* MirCreateActor(int32 ClassId)
{
	switch (ClassId)
	{
	case MirClass_Monigote: return new AMonigote();
	case MirClass_Door: return new ADoor();
	case MirClass_Key: return new AKey();
	default: return new AActor();
	}
}

void MirWire(int32, int32, int32) {}

int32 MirCall(int32 Function, int32 A, int32 B)
{
	switch (Function)
	{
	case 1: return MirGet<ADoor>(A)->CanOpen(MirGet<AMonigote>(B)) ? 1 : 0;
	case 10: MirGet<AMonigote>(A)->OnKeyPickedUp(MirGet<AKey>(B)); return 1;
	default: return 0;
	}
}
