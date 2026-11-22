// Memoria: un "bump allocator" sencillo. Cada escenario arranca con memoria limpia,
// así que nunca hace falta liberar de verdad.
#include "Mir/Core.h"

extern "C" unsigned char __heap_base;
static size_t GHeapTop = 0;

void* Mir::Malloc(size_t Size)
{
	if (GHeapTop == 0) { GHeapTop = (size_t)&__heap_base; }
	GHeapTop = (GHeapTop + 15) & ~(size_t)15;
	const size_t Result = GHeapTop;
	const size_t NewTop = Result + (Size ? Size : 1);
	const size_t CurrentBytes = (size_t)__builtin_wasm_memory_size(0) * 65536;
	if (NewTop > CurrentBytes)
	{
		const size_t PagesNeeded = (NewTop - CurrentBytes + 65535) / 65536;
		if (__builtin_wasm_memory_grow(0, PagesNeeded) == (size_t)-1)
		{
			Mir::Fatal("Out of memory: tu codigo ha pedido demasiada memoria (¿un bucle que no termina?)");
		}
	}
	GHeapTop = NewTop;
	return (void*)Result;
}

void* operator new(size_t Size) { return Mir::Malloc(Size); }
void* operator new[](size_t Size) { return Mir::Malloc(Size); }
void operator delete(void*) noexcept {}
void operator delete[](void*) noexcept {}
void operator delete(void*, size_t) noexcept {}
void operator delete[](void*, size_t) noexcept {}

extern "C" __attribute__((no_builtin("memcpy"))) void* memcpy(void* Dest, const void* Src, size_t Count)
{
	unsigned char* D = (unsigned char*)Dest;
	const unsigned char* S = (const unsigned char*)Src;
	while (Count--) { *D++ = *S++; }
	return Dest;
}

extern "C" __attribute__((no_builtin("memmove"))) void* memmove(void* Dest, const void* Src, size_t Count)
{
	unsigned char* D = (unsigned char*)Dest;
	const unsigned char* S = (const unsigned char*)Src;
	if (D < S) { while (Count--) { *D++ = *S++; } }
	else { D += Count; S += Count; while (Count--) { *--D = *--S; } }
	return Dest;
}

extern "C" __attribute__((no_builtin("memset"))) void* memset(void* Dest, int Value, size_t Count)
{
	unsigned char* D = (unsigned char*)Dest;
	while (Count--) { *D++ = (unsigned char)Value; }
	return Dest;
}

extern "C" __attribute__((no_builtin("memcmp"))) int memcmp(const void* A, const void* B, size_t Count)
{
	const unsigned char* X = (const unsigned char*)A;
	const unsigned char* Y = (const unsigned char*)B;
	for (size_t i = 0; i < Count; ++i)
	{
		if (X[i] != Y[i]) { return X[i] < Y[i] ? -1 : 1; }
	}
	return 0;
}

// Llamadas virtuales puras sin implementar.
extern "C" void __cxa_pure_virtual() { Mir::Fatal("Pure virtual function called"); }
// Destructores de objetos estáticos: nunca se ejecutan.
extern "C" int __cxa_atexit(void (*)(void*), void*, void*) { return 0; }
void* __dso_handle = nullptr;

extern "C" __attribute__((no_builtin("strlen"))) size_t strlen(const char* Str)
{
	size_t Len = 0;
	while (Str[Len]) { ++Len; }
	return Len;
}
