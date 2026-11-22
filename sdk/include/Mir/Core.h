// Make It Real — mini-Unreal SDK.
// Tipos básicos, macros de reflexión (vacías) y utilidades internas.
// En Unreal esto vive repartido por Core/Public/HAL y CoreUObject.
#pragma once

using size_t = __SIZE_TYPE__;

using int8 = signed char;
using uint8 = unsigned char;
using int16 = short;
using uint16 = unsigned short;
using int32 = int;
using uint32 = unsigned int;
using int64 = long long;
using uint64 = unsigned long long;
using TCHAR = char;
using ANSICHAR = char;

#define TEXT(x) x
#define FORCEINLINE inline
#define FORCENOINLINE __attribute__((noinline))
#define UE_ARRAY_COUNT(Array) (sizeof(Array) / sizeof((Array)[0]))

// Macros de reflexión: en Unreal las procesa UnrealHeaderTool. Aquí no hacen nada,
// pero las dejamos para que el código se escriba exactamente igual.
#define UCLASS(...)
#define USTRUCT(...)
#define UENUM(...)
#define UPROPERTY(...)
#define UFUNCTION(...)
#define UMETA(...)
#define GENERATED_BODY(...)
#define GENERATED_UCLASS_BODY(...)
#define GENERATED_USTRUCT_BODY(...)

#define MIR_IMPORT(Name) extern "C" __attribute__((import_module("ue"), import_name(#Name)))
#define MIR_EXPORT(Name) extern "C" __attribute__((export_name(#Name)))

// Placement new (normalmente viene de <new>).
inline void* operator new(size_t, void* Ptr) noexcept { return Ptr; }
inline void* operator new[](size_t, void* Ptr) noexcept { return Ptr; }

// std::initializer_list mínimo para poder escribir TArray<int32> A = {1, 2, 3};
namespace std
{
	template <class E>
	class initializer_list
	{
		const E* Begin_ = nullptr;
		size_t Size_ = 0;
		constexpr initializer_list(const E* B, size_t S) : Begin_(B), Size_(S) {}

	public:
		constexpr initializer_list() = default;
		constexpr size_t size() const { return Size_; }
		constexpr const E* begin() const { return Begin_; }
		constexpr const E* end() const { return Begin_ + Size_; }
	};
}

template <class T> struct TRemoveReference { using Type = T; };
template <class T> struct TRemoveReference<T&> { using Type = T; };
template <class T> struct TRemoveReference<T&&> { using Type = T; };
template <class T> constexpr typename TRemoveReference<T>::Type&& MoveTemp(T&& Obj) { return static_cast<typename TRemoveReference<T>::Type&&>(Obj); }
template <class T> constexpr T&& Forward(typename TRemoveReference<T>::Type& Obj) { return static_cast<T&&>(Obj); }
template <class T> inline void Swap(T& A, T& B) { T Tmp = MoveTemp(A); A = MoveTemp(B); B = MoveTemp(Tmp); }

namespace Mir
{
	[[noreturn]] void Fatal(const char* Message);
	[[noreturn]] void Fatalf(const char* Format, ...);
	void* Malloc(size_t Size);
	int32 StrLen(const char* Str);
	int32 VFormat(char* Buffer, int32 BufferSize, const char* Format, __builtin_va_list Args);
	int32 Format(char* Buffer, int32 BufferSize, const char* Format, ...);
	void Log(const char* Category, int32 Verbosity, const char* Format, ...);
}

#define check(Expr) do { if (!(Expr)) { ::Mir::Fatal("Assertion failed: " #Expr); } } while (0)
#define checkf(Expr, Format, ...) do { if (!(Expr)) { ::Mir::Fatalf("Assertion failed: " #Expr " | " Format, ##__VA_ARGS__); } } while (0)
#define checkNoEntry() ::Mir::Fatal("Enclosing block should never be called")
#define ensure(Expr) (!!(Expr))
#define ensureMsgf(Expr, ...) (!!(Expr))
#define verify(Expr) check(Expr)

#define UE_SMALL_NUMBER (1.e-8)
#define UE_KINDA_SMALL_NUMBER (1.e-4)
#define UE_BIG_NUMBER (3.4e+38f)
#define UE_PI (3.1415926535897932)
#define UE_DOUBLE_PI (3.141592653589793238462643383279502884197169399)
#define UE_HALF_PI (1.57079632679489661923)
#define UE_TWO_PI (6.28318530717958647692)
#define SMALL_NUMBER UE_SMALL_NUMBER
#define KINDA_SMALL_NUMBER UE_KINDA_SMALL_NUMBER
#define BIG_NUMBER UE_BIG_NUMBER
#define PI (3.1415926535897932f)
#define HALF_PI (1.57079632679489661923f)
#define TWO_PI (6.28318530717958647692f)
#define MAX_int32 ((int32)0x7fffffff)
#define MIN_int32 ((int32)0x80000000)
#define MAX_flt (3.402823466e+38F)
#define INDEX_NONE (-1)
