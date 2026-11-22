// Interno de Make It Real: lo que usa cada ejercicio (glue.cpp) para conectar
// el C++ del usuario con el mundo simulado. El usuario nunca ve este archivo.
#pragma once
#include "GameFramework/Actor.h"

// Debe coincidir con `Class` en crates/mir-server/src/sim/world.rs
enum EMirClass : int32
{
	MirClass_Actor = 0,
	MirClass_Monigote = 1,
	MirClass_Door = 2,
	MirClass_Key = 3,
	MirClass_Guard = 4,
	MirClass_TargetPoint = 5,
};

AActor* MirActor(int32 Handle);
inline int32 MirHandleOf(const AActor* Actor) { return Actor ? Actor->MirHandle : -1; }
template <class T> T* MirGet(int32 Handle) { return static_cast<T*>(MirActor(Handle)); }

// Cada glue.cpp define estas tres funciones.
AActor* MirCreateActor(int32 ClassId);
void MirWire(int32 Slot, int32 A, int32 B);
int32 MirCall(int32 Function, int32 A, int32 B);
