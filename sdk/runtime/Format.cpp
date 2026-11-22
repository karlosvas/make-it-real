// printf mínimo para UE_LOG, FString::Printf y los mensajes de error.
#include "CoreMinimal.h"

MIR_IMPORT(log) void MirHostLog(const char* Category, int32 CategoryLen, int32 Verbosity, const char* Message, int32 MessageLen);
MIR_IMPORT(fatal) void MirHostFatal(const char* Message, int32 MessageLen);

int32 Mir::StrLen(const char* Str)
{
	int32 Len = 0;
	while (Str[Len]) { ++Len; }
	return Len;
}

namespace
{
	struct FOut
	{
		char* Buffer;
		int32 Size;
		int32 Pos = 0;
		void Put(char C)
		{
			if (Pos < Size - 1) { Buffer[Pos] = C; }
			++Pos;
		}
	};

	void PutPadded(FOut& Out, const char* Text, int32 Len, int32 Width, bool bLeft, char Pad)
	{
		if (!bLeft) { for (int32 i = Len; i < Width; ++i) { Out.Put(Pad); } }
		for (int32 i = 0; i < Len; ++i) { Out.Put(Text[i]); }
		if (bLeft) { for (int32 i = Len; i < Width; ++i) { Out.Put(' '); } }
	}

	int32 FormatUnsigned(char* Tmp, uint64 Value, int32 Base, bool bUpper)
	{
		const char* Digits = bUpper ? "0123456789ABCDEF" : "0123456789abcdef";
		char Rev[32];
		int32 N = 0;
		do { Rev[N++] = Digits[Value % (uint64)Base]; Value /= (uint64)Base; } while (Value);
		for (int32 i = 0; i < N; ++i) { Tmp[i] = Rev[N - 1 - i]; }
		return N;
	}

	int32 FormatFloat(char* Tmp, double Value, int32 Precision, bool bTrim)
	{
		int32 N = 0;
		if (Value != Value) { Tmp[0] = 'n'; Tmp[1] = 'a'; Tmp[2] = 'n'; return 3; }
		if (Value < 0) { Tmp[N++] = '-'; Value = -Value; }
		if (Value > 1e18) { const char* Big = "inf"; for (int32 i = 0; i < 3; ++i) { Tmp[N++] = Big[i]; } return N; }
		if (Precision > 9) { Precision = 9; }
		uint64 Scale = 1;
		for (int32 i = 0; i < Precision; ++i) { Scale *= 10; }
		uint64 IntPart = (uint64)Value;
		double Frac = Value - (double)IntPart;
		uint64 FracPart = (uint64)(Frac * (double)Scale + 0.5);
		if (FracPart >= Scale) { FracPart -= Scale; IntPart += 1; }
		N += FormatUnsigned(Tmp + N, IntPart, 10, false);
		if (Precision > 0)
		{
			char FracDigits[16];
			for (int32 i = Precision - 1; i >= 0; --i) { FracDigits[i] = (char)('0' + FracPart % 10); FracPart /= 10; }
			int32 Count = Precision;
			if (bTrim) { while (Count > 0 && FracDigits[Count - 1] == '0') { --Count; } }
			if (Count > 0)
			{
				Tmp[N++] = '.';
				for (int32 i = 0; i < Count; ++i) { Tmp[N++] = FracDigits[i]; }
			}
		}
		return N;
	}
}

int32 Mir::VFormat(char* Buffer, int32 BufferSize, const char* Fmt, __builtin_va_list Args)
{
	FOut Out{Buffer, BufferSize};
	for (const char* P = Fmt; *P; ++P)
	{
		if (*P != '%') { Out.Put(*P); continue; }
		++P;
		bool bLeft = false, bPlus = false, bSpace = false;
		char Pad = ' ';
		for (;; ++P)
		{
			if (*P == '-') { bLeft = true; }
			else if (*P == '0') { Pad = '0'; }
			else if (*P == '+') { bPlus = true; }
			else if (*P == ' ') { bSpace = true; }
			else { break; }
		}
		int32 Width = 0;
		if (*P == '*') { Width = __builtin_va_arg(Args, int); ++P; }
		while (*P >= '0' && *P <= '9') { Width = Width * 10 + (*P++ - '0'); }
		int32 Precision = -1;
		if (*P == '.')
		{
			++P;
			Precision = 0;
			if (*P == '*') { Precision = __builtin_va_arg(Args, int); ++P; }
			while (*P >= '0' && *P <= '9') { Precision = Precision * 10 + (*P++ - '0'); }
		}
		int32 LongCount = 0;
		while (*P == 'l' || *P == 'h' || *P == 'z' || *P == 'j' || *P == 't' || *P == 'L')
		{
			if (*P == 'l') { ++LongCount; }
			++P;
		}
		char Tmp[64];
		int32 Len = 0;
		switch (*P)
		{
		case 'd':
		case 'i':
		{
			const int64 V = LongCount >= 2 ? __builtin_va_arg(Args, long long) : (LongCount == 1 ? __builtin_va_arg(Args, long) : __builtin_va_arg(Args, int));
			uint64 Abs = V < 0 ? (uint64)(-(V + 1)) + 1 : (uint64)V;
			if (V < 0) { Tmp[Len++] = '-'; }
			else if (bPlus) { Tmp[Len++] = '+'; }
			else if (bSpace) { Tmp[Len++] = ' '; }
			Len += FormatUnsigned(Tmp + Len, Abs, 10, false);
			PutPadded(Out, Tmp, Len, Width, bLeft, Pad);
			break;
		}
		case 'u':
		case 'x':
		case 'X':
		case 'o':
		{
			const uint64 V = LongCount >= 2 ? __builtin_va_arg(Args, unsigned long long) : (LongCount == 1 ? __builtin_va_arg(Args, unsigned long) : __builtin_va_arg(Args, unsigned int));
			Len = FormatUnsigned(Tmp, V, *P == 'u' ? 10 : (*P == 'o' ? 8 : 16), *P == 'X');
			PutPadded(Out, Tmp, Len, Width, bLeft, Pad);
			break;
		}
		case 'f':
		case 'F':
		case 'e':
		case 'g':
		case 'G':
		{
			const double V = __builtin_va_arg(Args, double);
			const bool bTrim = (*P == 'g' || *P == 'G');
			int32 Start = 0;
			if (V >= 0 && bPlus) { Tmp[Len++] = '+'; Start = 1; }
			Len = Start + FormatFloat(Tmp + Start, V, Precision < 0 ? 6 : Precision, bTrim);
			PutPadded(Out, Tmp, Len, Width, bLeft, Pad);
			break;
		}
		case 'c':
		{
			Tmp[0] = (char)__builtin_va_arg(Args, int);
			PutPadded(Out, Tmp, 1, Width, bLeft, ' ');
			break;
		}
		case 's':
		{
			const char* S = __builtin_va_arg(Args, const char*);
			if (!S) { S = "(null)"; }
			int32 SLen = Mir::StrLen(S);
			if (Precision >= 0 && Precision < SLen) { SLen = Precision; }
			PutPadded(Out, S, SLen, Width, bLeft, ' ');
			break;
		}
		case 'p':
		{
			Tmp[0] = '0'; Tmp[1] = 'x';
			Len = 2 + FormatUnsigned(Tmp + 2, (uint64)(size_t)__builtin_va_arg(Args, void*), 16, false);
			PutPadded(Out, Tmp, Len, Width, bLeft, ' ');
			break;
		}
		case '%': Out.Put('%'); break;
		case 0: --P; break;
		default: Out.Put('%'); Out.Put(*P); break;
		}
	}
	if (Out.Size > 0) { Out.Buffer[Out.Pos < Out.Size ? Out.Pos : Out.Size - 1] = 0; }
	return Out.Pos;
}

int32 Mir::Format(char* Buffer, int32 BufferSize, const char* Fmt, ...)
{
	__builtin_va_list Args;
	__builtin_va_start(Args, Fmt);
	const int32 Len = VFormat(Buffer, BufferSize, Fmt, Args);
	__builtin_va_end(Args);
	return Len;
}

void Mir::Log(const char* Category, int32 Verbosity, const char* Fmt, ...)
{
	char Buffer[1024];
	__builtin_va_list Args;
	__builtin_va_start(Args, Fmt);
	int32 Len = VFormat(Buffer, sizeof(Buffer), Fmt, Args);
	__builtin_va_end(Args);
	if (Len >= (int32)sizeof(Buffer)) { Len = sizeof(Buffer) - 1; }
	MirHostLog(Category, StrLen(Category), Verbosity, Buffer, Len);
	if (Verbosity == (int32)ELogVerbosity::Fatal) { Fatal(Buffer); }
}

void Mir::Fatal(const char* Message)
{
	MirHostFatal(Message, StrLen(Message));
	__builtin_trap();
}

void Mir::Fatalf(const char* Fmt, ...)
{
	char Buffer[512];
	__builtin_va_list Args;
	__builtin_va_start(Args, Fmt);
	VFormat(Buffer, sizeof(Buffer), Fmt, Args);
	__builtin_va_end(Args);
	Fatal(Buffer);
}

FString FString::Printf(const TCHAR* Fmt, ...)
{
	char Buffer[1024];
	__builtin_va_list Args;
	__builtin_va_start(Args, Fmt);
	::Mir::VFormat(Buffer, sizeof(Buffer), Fmt, Args);
	__builtin_va_end(Args);
	return FString(Buffer);
}

FLogCategoryBase LogTemp{"LogTemp"};
