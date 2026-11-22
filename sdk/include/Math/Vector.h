// FVector, FRotator y FColor con la API de Unreal Engine 5 (FVector usa double).
// Sistema de coordenadas de Unreal: X = delante, Y = derecha, Z = arriba. Unidades: centímetros.
#pragma once
#include "Math/UnrealMathUtility.h"

class FString;

struct FVector
{
	double X, Y, Z;

	constexpr FVector() : X(0), Y(0), Z(0) {}
	constexpr FVector(double InX, double InY, double InZ) : X(InX), Y(InY), Z(InZ) {}
	explicit constexpr FVector(double InF) : X(InF), Y(InF), Z(InF) {}

	static const FVector ZeroVector;
	static const FVector OneVector;
	static const FVector UpVector;
	static const FVector DownVector;
	static const FVector ForwardVector;
	static const FVector BackwardVector;
	static const FVector RightVector;
	static const FVector LeftVector;
	static const FVector XAxisVector;
	static const FVector YAxisVector;
	static const FVector ZAxisVector;

	constexpr FVector operator+(const FVector& V) const { return FVector(X + V.X, Y + V.Y, Z + V.Z); }
	constexpr FVector operator-(const FVector& V) const { return FVector(X - V.X, Y - V.Y, Z - V.Z); }
	constexpr FVector operator*(const FVector& V) const { return FVector(X * V.X, Y * V.Y, Z * V.Z); }
	constexpr FVector operator/(const FVector& V) const { return FVector(X / V.X, Y / V.Y, Z / V.Z); }
	constexpr FVector operator*(double Scale) const { return FVector(X * Scale, Y * Scale, Z * Scale); }
	constexpr FVector operator/(double Scale) const { return FVector(X / Scale, Y / Scale, Z / Scale); }
	constexpr FVector operator+(double Bias) const { return FVector(X + Bias, Y + Bias, Z + Bias); }
	constexpr FVector operator-(double Bias) const { return FVector(X - Bias, Y - Bias, Z - Bias); }
	constexpr FVector operator-() const { return FVector(-X, -Y, -Z); }
	FVector& operator+=(const FVector& V) { X += V.X; Y += V.Y; Z += V.Z; return *this; }
	FVector& operator-=(const FVector& V) { X -= V.X; Y -= V.Y; Z -= V.Z; return *this; }
	FVector& operator*=(const FVector& V) { X *= V.X; Y *= V.Y; Z *= V.Z; return *this; }
	FVector& operator*=(double Scale) { X *= Scale; Y *= Scale; Z *= Scale; return *this; }
	FVector& operator/=(double Scale) { X /= Scale; Y /= Scale; Z /= Scale; return *this; }
	constexpr bool operator==(const FVector& V) const { return X == V.X && Y == V.Y && Z == V.Z; }
	constexpr bool operator!=(const FVector& V) const { return !(*this == V); }
	double& operator[](int32 Index) { check(Index >= 0 && Index < 3); return Index == 0 ? X : (Index == 1 ? Y : Z); }
	double operator[](int32 Index) const { check(Index >= 0 && Index < 3); return Index == 0 ? X : (Index == 1 ? Y : Z); }

	/** Producto escalar (como en Unreal, también con el operador |). */
	constexpr double operator|(const FVector& V) const { return X * V.X + Y * V.Y + Z * V.Z; }
	/** Producto vectorial (como en Unreal, también con el operador ^). */
	constexpr FVector operator^(const FVector& V) const { return FVector(Y * V.Z - Z * V.Y, Z * V.X - X * V.Z, X * V.Y - Y * V.X); }
	double Dot(const FVector& V) const { return *this | V; }
	FVector Cross(const FVector& V) const { return *this ^ V; }

	static constexpr double DotProduct(const FVector& A, const FVector& B) { return A | B; }
	static constexpr FVector CrossProduct(const FVector& A, const FVector& B) { return A ^ B; }
	static double Dist(const FVector& A, const FVector& B) { return FMath::Sqrt(DistSquared(A, B)); }
	static double Distance(const FVector& A, const FVector& B) { return Dist(A, B); }
	static constexpr double DistSquared(const FVector& A, const FVector& B) { return FMath::Square(B.X - A.X) + FMath::Square(B.Y - A.Y) + FMath::Square(B.Z - A.Z); }
	static double Dist2D(const FVector& A, const FVector& B) { return FMath::Sqrt(DistSquared2D(A, B)); }
	static double DistXY(const FVector& A, const FVector& B) { return Dist2D(A, B); }
	static constexpr double DistSquared2D(const FVector& A, const FVector& B) { return FMath::Square(B.X - A.X) + FMath::Square(B.Y - A.Y); }
	static constexpr double DistSquaredXY(const FVector& A, const FVector& B) { return DistSquared2D(A, B); }

	double Size() const { return FMath::Sqrt(X * X + Y * Y + Z * Z); }
	double Length() const { return Size(); }
	constexpr double SizeSquared() const { return X * X + Y * Y + Z * Z; }
	constexpr double SquaredLength() const { return SizeSquared(); }
	double Size2D() const { return FMath::Sqrt(X * X + Y * Y); }
	constexpr double SizeSquared2D() const { return X * X + Y * Y; }

	FVector GetSafeNormal(double Tolerance = UE_SMALL_NUMBER) const
	{
		const double SquareSum = SizeSquared();
		if (SquareSum == 1.0) { return *this; }
		if (SquareSum < Tolerance) { return ZeroVector; }
		return *this * FMath::InvSqrt(SquareSum);
	}
	FVector GetSafeNormal2D(double Tolerance = UE_SMALL_NUMBER) const
	{
		const double SquareSum = X * X + Y * Y;
		if (SquareSum < Tolerance) { return ZeroVector; }
		const double Scale = FMath::InvSqrt(SquareSum);
		return FVector(X * Scale, Y * Scale, 0);
	}
	FVector GetUnsafeNormal() const { return *this * FMath::InvSqrt(SizeSquared()); }
	bool Normalize(double Tolerance = UE_SMALL_NUMBER)
	{
		const double SquareSum = SizeSquared();
		if (SquareSum > Tolerance) { *this *= FMath::InvSqrt(SquareSum); return true; }
		return false;
	}
	bool IsNormalized() const { return FMath::Abs(1.0 - SizeSquared()) < 0.01; }
	bool IsNearlyZero(double Tolerance = UE_KINDA_SMALL_NUMBER) const { return FMath::Abs(X) <= Tolerance && FMath::Abs(Y) <= Tolerance && FMath::Abs(Z) <= Tolerance; }
	constexpr bool IsZero() const { return X == 0 && Y == 0 && Z == 0; }
	bool Equals(const FVector& V, double Tolerance = UE_KINDA_SMALL_NUMBER) const { return FMath::Abs(X - V.X) <= Tolerance && FMath::Abs(Y - V.Y) <= Tolerance && FMath::Abs(Z - V.Z) <= Tolerance; }
	FVector GetClampedToMaxSize(double MaxSize) const
	{
		if (MaxSize < UE_KINDA_SMALL_NUMBER) { return ZeroVector; }
		const double VSq = SizeSquared();
		if (VSq > MaxSize * MaxSize) { return *this * (MaxSize * FMath::InvSqrt(VSq)); }
		return *this;
	}
	FVector GetAbs() const { return FVector(FMath::Abs(X), FMath::Abs(Y), FMath::Abs(Z)); }
	double GetMax() const { return FMath::Max(FMath::Max(X, Y), Z); }
	double GetMin() const { return FMath::Min(FMath::Min(X, Y), Z); }
	FVector RotateAngleAxis(double AngleDeg, const FVector& Axis) const;
	FRotator Rotation() const;
	FRotator ToOrientationRotator() const;
	FString ToString() const;
	static double CosineAngle2D(FVector A, FVector B)
	{
		A.Z = 0; B.Z = 0; A.Normalize(); B.Normalize();
		return A | B;
	}
};

inline constexpr FVector operator*(double Scale, const FVector& V) { return V * Scale; }

using FVector3d = FVector;

struct FVector2D
{
	double X, Y;
	constexpr FVector2D() : X(0), Y(0) {}
	constexpr FVector2D(double InX, double InY) : X(InX), Y(InY) {}
	constexpr FVector2D operator+(const FVector2D& V) const { return FVector2D(X + V.X, Y + V.Y); }
	constexpr FVector2D operator-(const FVector2D& V) const { return FVector2D(X - V.X, Y - V.Y); }
	constexpr FVector2D operator*(double S) const { return FVector2D(X * S, Y * S); }
	double Size() const { return FMath::Sqrt(X * X + Y * Y); }
	static constexpr double DotProduct(const FVector2D& A, const FVector2D& B) { return A.X * B.X + A.Y * B.Y; }
};

struct FRotator
{
	/** Rotación alrededor del eje Y (mirar arriba/abajo), en grados. */
	double Pitch;
	/** Rotación alrededor del eje Z (girar a izquierda/derecha), en grados. */
	double Yaw;
	/** Rotación alrededor del eje X (inclinar), en grados. */
	double Roll;

	constexpr FRotator() : Pitch(0), Yaw(0), Roll(0) {}
	constexpr FRotator(double InPitch, double InYaw, double InRoll) : Pitch(InPitch), Yaw(InYaw), Roll(InRoll) {}
	explicit constexpr FRotator(double InF) : Pitch(InF), Yaw(InF), Roll(InF) {}

	static const FRotator ZeroRotator;

	constexpr FRotator operator+(const FRotator& R) const { return FRotator(Pitch + R.Pitch, Yaw + R.Yaw, Roll + R.Roll); }
	constexpr FRotator operator-(const FRotator& R) const { return FRotator(Pitch - R.Pitch, Yaw - R.Yaw, Roll - R.Roll); }
	constexpr FRotator operator*(double Scale) const { return FRotator(Pitch * Scale, Yaw * Scale, Roll * Scale); }
	FRotator& operator+=(const FRotator& R) { Pitch += R.Pitch; Yaw += R.Yaw; Roll += R.Roll; return *this; }
	FRotator& operator-=(const FRotator& R) { Pitch -= R.Pitch; Yaw -= R.Yaw; Roll -= R.Roll; return *this; }
	FRotator& operator*=(double Scale) { Pitch *= Scale; Yaw *= Scale; Roll *= Scale; return *this; }
	constexpr bool operator==(const FRotator& R) const { return Pitch == R.Pitch && Yaw == R.Yaw && Roll == R.Roll; }
	constexpr bool operator!=(const FRotator& R) const { return !(*this == R); }

	/** Vector unitario hacia donde apunta esta rotación (su "delante"). */
	FVector Vector() const;
	FVector RotateVector(const FVector& V) const;
	FVector UnrotateVector(const FVector& V) const;
	FVector GetForwardVector() const { return Vector(); }
	FVector GetRightVector() const;
	FVector GetUpVector() const;

	static double NormalizeAxis(double Angle)
	{
		Angle = FMath::Fmod(Angle, 360.0);
		if (Angle < 0) { Angle += 360.0; }
		if (Angle > 180.0) { Angle -= 360.0; }
		return Angle;
	}
	static double ClampAxis(double Angle)
	{
		Angle = FMath::Fmod(Angle, 360.0);
		if (Angle < 0) { Angle += 360.0; }
		return Angle;
	}
	void Normalize() { Pitch = NormalizeAxis(Pitch); Yaw = NormalizeAxis(Yaw); Roll = NormalizeAxis(Roll); }
	FRotator GetNormalized() const { FRotator R = *this; R.Normalize(); return R; }
	FRotator Clamp() const { return FRotator(ClampAxis(Pitch), ClampAxis(Yaw), ClampAxis(Roll)); }
	FRotator GetInverse() const { return FRotator(-Pitch, -Yaw, -Roll); }
	bool IsNearlyZero(double Tolerance = UE_KINDA_SMALL_NUMBER) const
	{
		return FMath::Abs(NormalizeAxis(Pitch)) <= Tolerance && FMath::Abs(NormalizeAxis(Yaw)) <= Tolerance && FMath::Abs(NormalizeAxis(Roll)) <= Tolerance;
	}
	bool IsZero() const { return ClampAxis(Pitch) == 0 && ClampAxis(Yaw) == 0 && ClampAxis(Roll) == 0; }
	bool Equals(const FRotator& R, double Tolerance = UE_KINDA_SMALL_NUMBER) const
	{
		return FMath::Abs(NormalizeAxis(Pitch - R.Pitch)) <= Tolerance && FMath::Abs(NormalizeAxis(Yaw - R.Yaw)) <= Tolerance && FMath::Abs(NormalizeAxis(Roll - R.Roll)) <= Tolerance;
	}
	FString ToString() const;
};

using FRotator3d = FRotator;

struct FColor
{
	uint8 R, G, B, A;
	constexpr FColor() : R(0), G(0), B(0), A(255) {}
	constexpr FColor(uint8 InR, uint8 InG, uint8 InB, uint8 InA = 255) : R(InR), G(InG), B(InB), A(InA) {}
	constexpr uint32 ToPackedRGBA() const { return ((uint32)R << 24) | ((uint32)G << 16) | ((uint32)B << 8) | (uint32)A; }
	constexpr bool operator==(const FColor& C) const { return R == C.R && G == C.G && B == C.B && A == C.A; }

	static const FColor White;
	static const FColor Black;
	static const FColor Transparent;
	static const FColor Red;
	static const FColor Green;
	static const FColor Blue;
	static const FColor Yellow;
	static const FColor Cyan;
	static const FColor Magenta;
	static const FColor Orange;
	static const FColor Purple;
	static const FColor Turquoise;
	static const FColor Silver;
	static const FColor Emerald;
};

struct FLinearColor
{
	float R, G, B, A;
	constexpr FLinearColor() : R(0), G(0), B(0), A(1) {}
	constexpr FLinearColor(float InR, float InG, float InB, float InA = 1.f) : R(InR), G(InG), B(InB), A(InA) {}
	FColor ToFColor(bool /*bSRGB*/) const
	{
		return FColor((uint8)(FMath::Clamp(R, 0.f, 1.f) * 255.f), (uint8)(FMath::Clamp(G, 0.f, 1.f) * 255.f), (uint8)(FMath::Clamp(B, 0.f, 1.f) * 255.f), (uint8)(FMath::Clamp(A, 0.f, 1.f) * 255.f));
	}
	static const FLinearColor White;
	static const FLinearColor Black;
	static const FLinearColor Red;
	static const FLinearColor Green;
	static const FLinearColor Blue;
	static const FLinearColor Yellow;
};

// Implementaciones que necesitan ambos tipos.
inline FVector FRotator::Vector() const
{
	const double CP = FMath::Cos(FMath::DegreesToRadians(Pitch));
	const double SP = FMath::Sin(FMath::DegreesToRadians(Pitch));
	const double CY = FMath::Cos(FMath::DegreesToRadians(Yaw));
	const double SY = FMath::Sin(FMath::DegreesToRadians(Yaw));
	return FVector(CP * CY, CP * SY, SP);
}

inline FVector FRotator::RotateVector(const FVector& V) const
{
	const double SP = FMath::Sin(FMath::DegreesToRadians(Pitch)), CP = FMath::Cos(FMath::DegreesToRadians(Pitch));
	const double SY = FMath::Sin(FMath::DegreesToRadians(Yaw)), CY = FMath::Cos(FMath::DegreesToRadians(Yaw));
	const double SR = FMath::Sin(FMath::DegreesToRadians(Roll)), CR = FMath::Cos(FMath::DegreesToRadians(Roll));
	const FVector AxisX(CP * CY, CP * SY, SP);
	const FVector AxisY(SR * SP * CY - CR * SY, SR * SP * SY + CR * CY, -SR * CP);
	const FVector AxisZ(-(CR * SP * CY + SR * SY), CY * SR - CR * SP * SY, CR * CP);
	return AxisX * V.X + AxisY * V.Y + AxisZ * V.Z;
}

inline FVector FRotator::UnrotateVector(const FVector& V) const
{
	const FVector AxisX = RotateVector(FVector(1, 0, 0));
	const FVector AxisY = RotateVector(FVector(0, 1, 0));
	const FVector AxisZ = RotateVector(FVector(0, 0, 1));
	return FVector(V | AxisX, V | AxisY, V | AxisZ);
}

inline FVector FRotator::GetRightVector() const { return RotateVector(FVector(0, 1, 0)); }
inline FVector FRotator::GetUpVector() const { return RotateVector(FVector(0, 0, 1)); }

inline FRotator FVector::Rotation() const
{
	FRotator R;
	R.Yaw = FMath::RadiansToDegrees(FMath::Atan2(Y, X));
	R.Pitch = FMath::RadiansToDegrees(FMath::Atan2(Z, FMath::Sqrt(X * X + Y * Y)));
	R.Roll = 0;
	return R;
}
inline FRotator FVector::ToOrientationRotator() const { return Rotation(); }

inline FVector FVector::RotateAngleAxis(double AngleDeg, const FVector& Axis) const
{
	const double S = FMath::Sin(FMath::DegreesToRadians(AngleDeg));
	const double C = FMath::Cos(FMath::DegreesToRadians(AngleDeg));
	const double XX = Axis.X * Axis.X, YY = Axis.Y * Axis.Y, ZZ = Axis.Z * Axis.Z;
	const double XY = Axis.X * Axis.Y, YZ = Axis.Y * Axis.Z, ZX = Axis.Z * Axis.X;
	const double XS = Axis.X * S, YS = Axis.Y * S, ZS = Axis.Z * S;
	const double OMC = 1.0 - C;
	return FVector(
		(OMC * XX + C) * X + (OMC * XY - ZS) * Y + (OMC * ZX + YS) * Z,
		(OMC * XY + ZS) * X + (OMC * YY + C) * Y + (OMC * YZ - XS) * Z,
		(OMC * ZX - YS) * X + (OMC * YZ + XS) * Y + (OMC * ZZ + C) * Z);
}

inline FVector FMath::VInterpTo(const FVector& Current, const FVector& Target, float DeltaTime, float InterpSpeed)
{
	if (InterpSpeed <= 0.f) { return Target; }
	const FVector Dist = Target - Current;
	if (Dist.SizeSquared() < UE_KINDA_SMALL_NUMBER) { return Target; }
	return Current + Dist * FMath::Clamp(DeltaTime * InterpSpeed, 0.f, 1.f);
}

inline FVector FMath::VInterpConstantTo(const FVector& Current, const FVector& Target, float DeltaTime, float InterpSpeed)
{
	const FVector Delta = Target - Current;
	const double DeltaM = Delta.Size();
	const double MaxStep = InterpSpeed * DeltaTime;
	if (DeltaM > MaxStep)
	{
		if (MaxStep > 0.f) { return Current + Delta / DeltaM * MaxStep; }
		return Current;
	}
	return Target;
}

inline FRotator FMath::RInterpTo(const FRotator& Current, const FRotator& Target, float DeltaTime, float InterpSpeed)
{
	if (DeltaTime == 0.f || Current == Target) { return Current; }
	if (InterpSpeed <= 0.f) { return Target; }
	const FRotator Delta = (Target - Current).GetNormalized();
	if (Delta.IsNearlyZero()) { return Target; }
	const FRotator DeltaMove = Delta * FMath::Clamp(InterpSpeed * DeltaTime, 0.f, 1.f);
	return (Current + DeltaMove).GetNormalized();
}

inline FRotator FMath::RInterpConstantTo(const FRotator& Current, const FRotator& Target, float DeltaTime, float InterpSpeed)
{
	if (DeltaTime == 0.f || Current == Target) { return Current; }
	if (InterpSpeed <= 0.f) { return Target; }
	const double Step = InterpSpeed * DeltaTime;
	const FRotator Delta = (Target - Current).GetNormalized();
	FRotator Result = Current;
	Result.Pitch += FMath::Clamp(Delta.Pitch, -Step, Step);
	Result.Yaw += FMath::Clamp(Delta.Yaw, -Step, Step);
	Result.Roll += FMath::Clamp(Delta.Roll, -Step, Step);
	return Result.GetNormalized();
}
