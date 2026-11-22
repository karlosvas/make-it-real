// TArray: array dinámico con la API de Unreal (subconjunto).
#pragma once
#include "Mir/Core.h"

template <class T>
class TArray
{
public:
	using ElementType = T;

	TArray() = default;
	TArray(std::initializer_list<T> List)
	{
		Reserve((int32)List.size());
		for (const T& Item : List) { Add(Item); }
	}
	TArray(const TArray& Other) { CopyFrom(Other); }
	TArray& operator=(const TArray& Other)
	{
		if (this != &Other) { Empty(); CopyFrom(Other); }
		return *this;
	}
	~TArray() { Empty(); }

	int32 Num() const { return ArrayNum; }
	int32 Max() const { return ArrayMax; }
	bool IsEmpty() const { return ArrayNum == 0; }
	bool IsValidIndex(int32 Index) const { return Index >= 0 && Index < ArrayNum; }
	T* GetData() { return Data; }
	const T* GetData() const { return Data; }

	T& operator[](int32 Index) { RangeCheck(Index); return Data[Index]; }
	const T& operator[](int32 Index) const { RangeCheck(Index); return Data[Index]; }

	T& Last(int32 IndexFromTheEnd = 0) { RangeCheck(ArrayNum - IndexFromTheEnd - 1); return Data[ArrayNum - IndexFromTheEnd - 1]; }
	const T& Last(int32 IndexFromTheEnd = 0) const { RangeCheck(ArrayNum - IndexFromTheEnd - 1); return Data[ArrayNum - IndexFromTheEnd - 1]; }
	T& Top() { return Last(); }

	int32 Add(const T& Item)
	{
		EnsureCapacity(ArrayNum + 1);
		new (&Data[ArrayNum]) T(Item);
		return ArrayNum++;
	}
	int32 Emplace(const T& Item) { return Add(Item); }
	int32 AddUnique(const T& Item)
	{
		const int32 Found = Find(Item);
		return Found != INDEX_NONE ? Found : Add(Item);
	}
	int32 AddDefaulted(int32 Count = 1)
	{
		EnsureCapacity(ArrayNum + Count);
		const int32 First = ArrayNum;
		for (int32 i = 0; i < Count; ++i) { new (&Data[ArrayNum++]) T(); }
		return First;
	}
	void Push(const T& Item) { Add(Item); }
	T Pop()
	{
		RangeCheck(ArrayNum - 1);
		T Result = Data[ArrayNum - 1];
		RemoveAt(ArrayNum - 1);
		return Result;
	}
	void Insert(const T& Item, int32 Index)
	{
		checkf(Index >= 0 && Index <= ArrayNum, "Insert: index %d out of bounds (size %d)", Index, ArrayNum);
		EnsureCapacity(ArrayNum + 1);
		new (&Data[ArrayNum]) T();
		for (int32 i = ArrayNum; i > Index; --i) { Data[i] = Data[i - 1]; }
		Data[Index] = Item;
		++ArrayNum;
	}
	void Append(const TArray& Other)
	{
		for (int32 i = 0; i < Other.Num(); ++i) { Add(Other[i]); }
	}

	int32 Find(const T& Item) const
	{
		for (int32 i = 0; i < ArrayNum; ++i) { if (Data[i] == Item) { return i; } }
		return INDEX_NONE;
	}
	bool Find(const T& Item, int32& OutIndex) const { OutIndex = Find(Item); return OutIndex != INDEX_NONE; }
	int32 IndexOfByKey(const T& Item) const { return Find(Item); }
	bool Contains(const T& Item) const { return Find(Item) != INDEX_NONE; }

	void RemoveAt(int32 Index, int32 Count = 1)
	{
		checkf(Index >= 0 && Count >= 0 && Index + Count <= ArrayNum, "RemoveAt: index %d (count %d) out of bounds (size %d)", Index, Count, ArrayNum);
		for (int32 i = Index; i + Count < ArrayNum; ++i) { Data[i] = Data[i + Count]; }
		for (int32 i = ArrayNum - Count; i < ArrayNum; ++i) { Data[i].~T(); }
		ArrayNum -= Count;
	}
	void RemoveAtSwap(int32 Index)
	{
		RangeCheck(Index);
		Data[Index] = Data[ArrayNum - 1];
		RemoveAt(ArrayNum - 1);
	}
	int32 Remove(const T& Item)
	{
		int32 Removed = 0;
		for (int32 i = ArrayNum - 1; i >= 0; --i)
		{
			if (Data[i] == Item) { RemoveAt(i); ++Removed; }
		}
		return Removed;
	}
	int32 RemoveSingle(const T& Item)
	{
		const int32 Index = Find(Item);
		if (Index == INDEX_NONE) { return 0; }
		RemoveAt(Index);
		return 1;
	}
	void Empty(int32 Slack = 0)
	{
		for (int32 i = 0; i < ArrayNum; ++i) { Data[i].~T(); }
		ArrayNum = 0;
		if (Slack > ArrayMax) { Reserve(Slack); }
	}
	void Reset() { Empty(); }
	void Reserve(int32 Number) { if (Number > ArrayMax) { Grow(Number); } }
	void SetNum(int32 NewNum)
	{
		if (NewNum > ArrayNum) { AddDefaulted(NewNum - ArrayNum); }
		else if (NewNum < ArrayNum) { RemoveAt(NewNum, ArrayNum - NewNum); }
	}
	void Swap(int32 A, int32 B) { RangeCheck(A); RangeCheck(B); ::Swap(Data[A], Data[B]); }

	template <class Predicate>
	T* FindByPredicate(Predicate Pred)
	{
		for (int32 i = 0; i < ArrayNum; ++i) { if (Pred(Data[i])) { return &Data[i]; } }
		return nullptr;
	}
	template <class Predicate>
	bool ContainsByPredicate(Predicate Pred) const
	{
		for (int32 i = 0; i < ArrayNum; ++i) { if (Pred(Data[i])) { return true; } }
		return false;
	}
	template <class Predicate>
	TArray FilterByPredicate(Predicate Pred) const
	{
		TArray Result;
		for (int32 i = 0; i < ArrayNum; ++i) { if (Pred(Data[i])) { Result.Add(Data[i]); } }
		return Result;
	}
	/** Ordena de menor a mayor (inserción; suficiente para arrays pequeños). */
	void Sort()
	{
		Sort([](const T& A, const T& B) { return A < B; });
	}
	template <class Predicate>
	void Sort(Predicate Less)
	{
		for (int32 i = 1; i < ArrayNum; ++i)
		{
			for (int32 j = i; j > 0 && Less(Data[j], Data[j - 1]); --j) { ::Swap(Data[j], Data[j - 1]); }
		}
	}

	T* begin() { return Data; }
	T* end() { return Data + ArrayNum; }
	const T* begin() const { return Data; }
	const T* end() const { return Data + ArrayNum; }

	bool operator==(const TArray& Other) const
	{
		if (ArrayNum != Other.ArrayNum) { return false; }
		for (int32 i = 0; i < ArrayNum; ++i) { if (!(Data[i] == Other.Data[i])) { return false; } }
		return true;
	}

private:
	T* Data = nullptr;
	int32 ArrayNum = 0;
	int32 ArrayMax = 0;

	void RangeCheck(int32 Index) const
	{
		if (Index < 0 || Index >= ArrayNum)
		{
			::Mir::Fatalf("Array index out of bounds: %d from an array of size %d", Index, ArrayNum);
		}
	}
	void EnsureCapacity(int32 Needed)
	{
		if (Needed > ArrayMax) { Grow(Needed < 4 ? 4 : (Needed > ArrayMax * 2 ? Needed : ArrayMax * 2)); }
	}
	void Grow(int32 NewMax)
	{
		T* NewData = (T*)::Mir::Malloc(sizeof(T) * (size_t)NewMax);
		for (int32 i = 0; i < ArrayNum; ++i)
		{
			new (&NewData[i]) T(Data[i]);
			Data[i].~T();
		}
		Data = NewData;
		ArrayMax = NewMax;
	}
	void CopyFrom(const TArray& Other)
	{
		Reserve(Other.ArrayNum);
		for (int32 i = 0; i < Other.ArrayNum; ++i) { Add(Other.Data[i]); }
	}
};
