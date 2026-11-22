// FString mínimo con la API de Unreal.
#pragma once
#include "Mir/Core.h"

class FString
{
public:
	FString() = default;
	FString(const TCHAR* Str) { Assign(Str, Str ? ::Mir::StrLen(Str) : 0); }
	FString(const FString& Other) { Assign(Other.Data, Other.Length); }
	FString& operator=(const FString& Other)
	{
		if (this != &Other) { Assign(Other.Data, Other.Length); }
		return *this;
	}
	FString& operator=(const TCHAR* Str)
	{
		Assign(Str, Str ? ::Mir::StrLen(Str) : 0);
		return *this;
	}

	/** Puntero a los caracteres, para usar con %s en UE_LOG: *MiString */
	const TCHAR* operator*() const { return Data ? Data : ""; }
	int32 Len() const { return Length; }
	bool IsEmpty() const { return Length == 0; }

	FString operator+(const FString& Other) const
	{
		FString Result;
		Result.Concat(Data, Length, Other.Data, Other.Length);
		return Result;
	}
	FString operator+(const TCHAR* Other) const { return *this + FString(Other); }
	FString& operator+=(const FString& Other)
	{
		FString Result;
		Result.Concat(Data, Length, Other.Data, Other.Length);
		*this = Result;
		return *this;
	}
	FString& operator+=(const TCHAR* Other) { return *this += FString(Other); }
	FString& AppendInt(int32 Value) { return *this += FromInt(Value); }

	bool Equals(const FString& Other) const
	{
		if (Length != Other.Length) { return false; }
		for (int32 i = 0; i < Length; ++i) { if (Data[i] != Other.Data[i]) { return false; } }
		return true;
	}
	bool operator==(const FString& Other) const { return Equals(Other); }
	bool operator!=(const FString& Other) const { return !Equals(Other); }
	bool Contains(const FString& Sub) const
	{
		if (Sub.Length == 0) { return true; }
		for (int32 i = 0; i + Sub.Length <= Length; ++i)
		{
			int32 j = 0;
			while (j < Sub.Length && Data[i + j] == Sub.Data[j]) { ++j; }
			if (j == Sub.Length) { return true; }
		}
		return false;
	}

	static FString Printf(const TCHAR* Format, ...);
	static FString FromInt(int32 Value) { return Printf("%d", Value); }
	static FString SanitizeFloat(double Value) { return Printf("%g", Value); }

private:
	TCHAR* Data = nullptr;
	int32 Length = 0;

	void Assign(const TCHAR* Src, int32 Len)
	{
		Data = (TCHAR*)::Mir::Malloc((size_t)Len + 1);
		for (int32 i = 0; i < Len; ++i) { Data[i] = Src[i]; }
		Data[Len] = 0;
		Length = Len;
	}
	void Concat(const TCHAR* A, int32 LenA, const TCHAR* B, int32 LenB)
	{
		Data = (TCHAR*)::Mir::Malloc((size_t)(LenA + LenB) + 1);
		for (int32 i = 0; i < LenA; ++i) { Data[i] = A[i]; }
		for (int32 i = 0; i < LenB; ++i) { Data[LenA + i] = B[i]; }
		Data[LenA + LenB] = 0;
		Length = LenA + LenB;
	}
};

inline FString operator+(const TCHAR* A, const FString& B) { return FString(A) + B; }

/** FName y FText se comportan como FString en este mini-Unreal. */
using FName = FString;
using FText = FString;
