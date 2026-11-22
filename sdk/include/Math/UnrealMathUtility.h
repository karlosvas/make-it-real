// FMath: utilidades matemáticas con la misma API que Unreal.
#pragma once
#include "Mir/Core.h"

MIR_IMPORT(sin) double MirSin(double);
MIR_IMPORT(cos) double MirCos(double);
MIR_IMPORT(tan) double MirTan(double);
MIR_IMPORT(asin) double MirAsin(double);
MIR_IMPORT(acos) double MirAcos(double);
MIR_IMPORT(atan) double MirAtan(double);
MIR_IMPORT(atan2) double MirAtan2(double, double);
MIR_IMPORT(pow) double MirPow(double, double);
MIR_IMPORT(exp) double MirExp(double);
MIR_IMPORT(ln) double MirLog(double);
MIR_IMPORT(fmod) double MirFmod(double, double);

struct FVector;
struct FRotator;

struct FMath
{
	template <class T> static constexpr T Abs(const T A) { return (A < (T)0) ? -A : A; }
	template <class T> static constexpr T Sign(const T A) { return (A > (T)0) ? (T)1 : ((A < (T)0) ? (T)-1 : (T)0); }
	template <class T> static constexpr T Square(const T A) { return A * A; }
	template <class T> static constexpr T Cube(const T A) { return A * A * A; }

	template <class A, class B> static constexpr auto Min(const A X, const B Y) -> decltype(X + Y) { return (X <= Y) ? X : Y; }
	template <class A, class B> static constexpr auto Max(const A X, const B Y) -> decltype(X + Y) { return (X >= Y) ? X : Y; }
	template <class T> static constexpr T Min3(const T A, const T B, const T C) { return Min(Min(A, B), C); }
	template <class T> static constexpr T Max3(const T A, const T B, const T C) { return Max(Max(A, B), C); }
	template <class T, class L, class H> static constexpr T Clamp(const T X, const L Lo, const H Hi)
	{
		return X < (T)Lo ? (T)Lo : (X < (T)Hi ? X : (T)Hi);
	}
	template <class T> static constexpr bool IsWithin(const T& Value, const T& Lo, const T& Hi) { return (Value >= Lo) && (Value < Hi); }
	template <class T> static constexpr bool IsWithinInclusive(const T& Value, const T& Lo, const T& Hi) { return (Value >= Lo) && (Value <= Hi); }

	static float Sqrt(float X) { return __builtin_sqrtf(X); }
	static double Sqrt(double X) { return __builtin_sqrt(X); }
	static float InvSqrt(float X) { return 1.f / __builtin_sqrtf(X); }
	static double InvSqrt(double X) { return 1.0 / __builtin_sqrt(X); }

	static float Sin(float X) { return (float)MirSin(X); }
	static double Sin(double X) { return MirSin(X); }
	static float Cos(float X) { return (float)MirCos(X); }
	static double Cos(double X) { return MirCos(X); }
	static float Tan(float X) { return (float)MirTan(X); }
	static double Tan(double X) { return MirTan(X); }
	static float Asin(float X) { return (float)MirAsin(Clamp(X, -1.f, 1.f)); }
	static double Asin(double X) { return MirAsin(Clamp(X, -1.0, 1.0)); }
	static float Acos(float X) { return (float)MirAcos(Clamp(X, -1.f, 1.f)); }
	static double Acos(double X) { return MirAcos(Clamp(X, -1.0, 1.0)); }
	static float Atan(float X) { return (float)MirAtan(X); }
	static double Atan(double X) { return MirAtan(X); }
	static float Atan2(float Y, float X) { return (float)MirAtan2(Y, X); }
	static double Atan2(double Y, double X) { return MirAtan2(Y, X); }
	static float Pow(float A, float B) { return (float)MirPow(A, B); }
	static double Pow(double A, double B) { return MirPow(A, B); }
	static float Exp(float X) { return (float)MirExp(X); }
	static double Exp(double X) { return MirExp(X); }
	static float Loge(float X) { return (float)MirLog(X); }
	static double Loge(double X) { return MirLog(X); }
	static float Fmod(float X, float Y) { return (float)MirFmod(X, Y); }
	static double Fmod(double X, double Y) { return MirFmod(X, Y); }
	static void SinCos(float* S, float* C, float Value) { *S = Sin(Value); *C = Cos(Value); }
	static void SinCos(double* S, double* C, double Value) { *S = Sin(Value); *C = Cos(Value); }

	static float Floor(float X) { return __builtin_floorf(X); }
	static double Floor(double X) { return __builtin_floor(X); }
	static float CeilToFloat(float X) { return __builtin_ceilf(X); }
	static double CeilToDouble(double X) { return __builtin_ceil(X); }
	static float FloorToFloat(float X) { return __builtin_floorf(X); }
	static double FloorToDouble(double X) { return __builtin_floor(X); }
	static int32 FloorToInt(double X) { return (int32)__builtin_floor(X); }
	static int32 CeilToInt(double X) { return (int32)__builtin_ceil(X); }
	static int32 RoundToInt(double X) { return (int32)__builtin_floor(X + 0.5); }
	static float RoundToFloat(float X) { return __builtin_floorf(X + 0.5f); }
	static int32 TruncToInt(double X) { return (int32)X; }
	static float Frac(float X) { return X - __builtin_floorf(X); }

	template <class T> static constexpr auto DegreesToRadians(const T Deg) -> decltype(Deg * 1.0) { return Deg * (UE_DOUBLE_PI / 180.0); }
	static constexpr float DegreesToRadians(const float Deg) { return Deg * (PI / 180.f); }
	template <class T> static constexpr auto RadiansToDegrees(const T Rad) -> decltype(Rad * 1.0) { return Rad * (180.0 / UE_DOUBLE_PI); }
	static constexpr float RadiansToDegrees(const float Rad) { return Rad * (180.f / PI); }

	template <class T, class U> static constexpr auto Lerp(const T& A, const T& B, const U& Alpha) -> T { return (T)(A + Alpha * (B - A)); }
	template <class T> static constexpr T GetRangePct(T MinValue, T MaxValue, T Value)
	{
		const T Divisor = MaxValue - MinValue;
		return (Abs(Divisor) < (T)UE_SMALL_NUMBER) ? (Value >= MaxValue ? (T)1 : (T)0) : (Value - MinValue) / Divisor;
	}
	template <class T> static T GetMappedRangeValueClamped(T InMin, T InMax, T OutMin, T OutMax, T Value)
	{
		const T Pct = Clamp(GetRangePct(InMin, InMax, Value), (T)0, (T)1);
		return Lerp(OutMin, OutMax, Pct);
	}

	template <class A, class B> static constexpr bool IsNearlyEqual(A X, B Y, double Tolerance = UE_SMALL_NUMBER) { return Abs((double)X - (double)Y) <= Tolerance; }
	template <class T> static constexpr bool IsNearlyZero(T X, double Tolerance = UE_SMALL_NUMBER) { return Abs((double)X) <= Tolerance; }

	// Interpolaciones (mismo comportamiento que Unreal).
	template <class C, class T, class D, class S>
	static auto FInterpTo(C Current, T Target, D DeltaTime, S InterpSpeed) -> decltype(Current + Target)
	{
		using R = decltype(Current + Target);
		if (InterpSpeed <= 0) { return (R)Target; }
		const R Dist = (R)Target - (R)Current;
		if (Dist * Dist < (R)UE_SMALL_NUMBER) { return (R)Target; }
		const R DeltaMove = Dist * Clamp((R)(DeltaTime * InterpSpeed), (R)0, (R)1);
		return (R)Current + DeltaMove;
	}
	template <class C, class T, class D, class S>
	static auto FInterpConstantTo(C Current, T Target, D DeltaTime, S InterpSpeed) -> decltype(Current + Target)
	{
		using R = decltype(Current + Target);
		const R Dist = (R)Target - (R)Current;
		if (Dist * Dist < (R)UE_SMALL_NUMBER) { return (R)Target; }
		const R Step = (R)(InterpSpeed * DeltaTime);
		return (R)Current + Clamp(Dist, -Step, Step);
	}
	static FVector VInterpTo(const FVector& Current, const FVector& Target, float DeltaTime, float InterpSpeed);
	static FVector VInterpConstantTo(const FVector& Current, const FVector& Target, float DeltaTime, float InterpSpeed);
	static FRotator RInterpTo(const FRotator& Current, const FRotator& Target, float DeltaTime, float InterpSpeed);
	static FRotator RInterpConstantTo(const FRotator& Current, const FRotator& Target, float DeltaTime, float InterpSpeed);

	// Aleatorios deterministas (misma semilla en cada escenario).
	static int32 Rand();
	static float FRand();
	static int32 RandRange(int32 Min, int32 Max);
	static float FRandRange(float Min, float Max);
	static bool RandBool();
	static void RandInit(int32 Seed);

	// Normaliza un ángulo en grados a (-180, 180].
	template <class T> static T UnwindDegrees(T A)
	{
		while (A > (T)180) { A -= (T)360; }
		while (A < (T)-180) { A += (T)360; }
		return A;
	}
	template <class T> static T FindDeltaAngleDegrees(T A1, T A2) { return UnwindDegrees(A2 - A1); }
};
