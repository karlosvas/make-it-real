// AActor, componentes, UWorld y FTimerManager (subconjunto de la API de Unreal).
#pragma once
#include "CoreMinimal.h"

class AActor;
class UWorld;
class FTimerManager;

/** Base de todo objeto de Unreal. */
class UObject
{
public:
	using Super = UObject;
	virtual ~UObject() = default;
};

struct FActorTickFunction
{
	bool bCanEverTick = true;
	bool bStartWithTickEnabled = true;
	float TickInterval = 0.f;
	void SetTickFunctionEnable(bool bEnabled) { bStartWithTickEnabled = bEnabled; }
};

class UActorComponent : public UObject
{
public:
	using Super = UActorComponent;
	AActor* GetOwner() const { return MirOwner; }

	// Interno de Make It Real.
	AActor* MirOwner = nullptr;
	int32 MirIndex = 0;
};

class USceneComponent : public UActorComponent
{
public:
	using Super = USceneComponent;

	void SetRelativeRotation(const FRotator& NewRotation);
	FRotator GetRelativeRotation() const;
	void AddRelativeRotation(const FRotator& DeltaRotation) { SetRelativeRotation(GetRelativeRotation() + DeltaRotation); }
	void SetupAttachment(USceneComponent* /*Parent*/) {}
	FVector GetComponentLocation() const;
};

class UPrimitiveComponent : public USceneComponent
{
public:
	using Super = UPrimitiveComponent;
};

class UStaticMeshComponent : public UPrimitiveComponent
{
public:
	using Super = UStaticMeshComponent;
};

/** Un actor: cualquier cosa que existe en el mundo (monigote, puerta, llave...). */
class AActor : public UObject
{
public:
	using Super = AActor;

	AActor() = default;

	/** Se llama una vez cuando empieza la partida. */
	virtual void BeginPlay() {}
	/** Se llama en cada frame. DeltaSeconds = tiempo desde el frame anterior. */
	virtual void Tick(float DeltaSeconds) {}

	FVector GetActorLocation() const;
	bool SetActorLocation(const FVector& NewLocation, bool bSweep = false);
	FRotator GetActorRotation() const;
	bool SetActorRotation(const FRotator& NewRotation);
	bool SetActorLocationAndRotation(const FVector& NewLocation, const FRotator& NewRotation)
	{
		SetActorLocation(NewLocation);
		return SetActorRotation(NewRotation);
	}
	void AddActorWorldOffset(const FVector& DeltaLocation, bool bSweep = false) { SetActorLocation(GetActorLocation() + DeltaLocation, bSweep); }
	void AddActorWorldRotation(const FRotator& DeltaRotation) { SetActorRotation(GetActorRotation() + DeltaRotation); }
	void AddActorLocalRotation(const FRotator& DeltaRotation) { SetActorRotation(GetActorRotation() + DeltaRotation); }

	/** Vector unitario hacia el "delante" del actor (su eje X local). */
	FVector GetActorForwardVector() const;
	/** Vector unitario hacia la derecha del actor (su eje Y local). */
	FVector GetActorRightVector() const;
	/** Vector unitario hacia arriba del actor (su eje Z local). */
	FVector GetActorUpVector() const;

	/** Distancia (en cm) entre este actor y otro. */
	double GetDistanceTo(const AActor* OtherActor) const;
	double GetHorizontalDistanceTo(const AActor* OtherActor) const;
	virtual FVector GetVelocity() const;

	bool Destroy();
	bool IsHidden() const;
	void SetActorHiddenInGame(bool bNewHidden);
	FString GetName() const;
	FString GetActorNameOrLabel() const { return GetName(); }

	UWorld* GetWorld() const;
	FTimerManager& GetWorldTimerManager() const;

	template <class T>
	T* CreateDefaultSubobject(const TCHAR* /*SubobjectName*/)
	{
		T* Component = new T();
		Component->MirOwner = this;
		Component->MirIndex = MirNumComponents++;
		return Component;
	}
	USceneComponent* GetRootComponent() const { return RootComponent; }
	void SetRootComponent(USceneComponent* NewRoot) { RootComponent = NewRoot; }

	FActorTickFunction PrimaryActorTick;

	// Interno de Make It Real: identificador del actor en el mundo simulado.
	int32 MirHandle = -1;
	int32 MirNumComponents = 0;

protected:
	USceneComponent* RootComponent = nullptr;
};

/** Manejador de un temporizador (FTimerHandle en Unreal). */
struct FTimerHandle
{
	uint64 Handle = 0;
	bool IsValid() const { return Handle != 0; }
	void Invalidate() { Handle = 0; }
	bool operator==(const FTimerHandle& Other) const { return Handle == Other.Handle; }
};

struct FMirTimerCallback
{
	virtual ~FMirTimerCallback() = default;
	virtual void Execute() = 0;
};

template <class UserClass>
struct TMirMethodCallback : FMirTimerCallback
{
	UserClass* Object;
	void (UserClass::*Method)();
	TMirMethodCallback(UserClass* InObject, void (UserClass::*InMethod)()) : Object(InObject), Method(InMethod) {}
	void Execute() override { (Object->*Method)(); }
};

template <class Functor>
struct TMirLambdaCallback : FMirTimerCallback
{
	Functor Func;
	explicit TMirLambdaCallback(Functor InFunc) : Func(InFunc) {}
	void Execute() override { Func(); }
};

/** Gestor de temporizadores: GetWorldTimerManager().SetTimer(...) */
class FTimerManager
{
public:
	/**
	 * Llama a InMethod de InObj dentro de InRate segundos.
	 * Si bLoop es true, se repite cada InRate segundos.
	 * Si el handle ya tenía un temporizador activo, se reemplaza (igual que en Unreal).
	 */
	template <class UserClass>
	void SetTimer(FTimerHandle& InOutHandle, UserClass* InObj, void (UserClass::*InMethod)(), float InRate, bool bLoop = false, float InFirstDelay = -1.f)
	{
		SetTimerInternal(InOutHandle, new TMirMethodCallback<UserClass>(InObj, InMethod), InRate, bLoop, InFirstDelay);
	}
	/** Versión con lambda (en Unreal sería FTimerDelegate::CreateLambda). */
	template <class Functor>
	void SetTimerLambda(FTimerHandle& InOutHandle, Functor Func, float InRate, bool bLoop = false, float InFirstDelay = -1.f)
	{
		SetTimerInternal(InOutHandle, new TMirLambdaCallback<Functor>(Func), InRate, bLoop, InFirstDelay);
	}
	void ClearTimer(FTimerHandle& InHandle);
	bool IsTimerActive(FTimerHandle InHandle) const;
	float GetTimerRemaining(FTimerHandle InHandle) const;
	float GetTimerElapsed(FTimerHandle InHandle) const;
	float GetTimerRate(FTimerHandle InHandle) const;

	// Interno.
	void MirTick(float DeltaTime);

private:
	void SetTimerInternal(FTimerHandle& InOutHandle, FMirTimerCallback* Callback, float InRate, bool bLoop, float InFirstDelay);
};

class UWorld : public UObject
{
public:
	using Super = UWorld;
	/** Segundos desde que empezó la partida. */
	double GetTimeSeconds() const;
	/** Duración del frame actual en segundos. */
	float GetDeltaSeconds() const;
	FTimerManager& GetTimerManager() const;
};

/** GEngine->AddOnScreenDebugMessage(...) pinta texto encima del visor. */
class UEngine
{
public:
	void AddOnScreenDebugMessage(int32 Key, float TimeToDisplay, FColor DisplayColor, const FString& DebugMessage, bool bNewerOnTop = true, const FVector2D& TextScale = FVector2D(1, 1));
};
extern UEngine* GEngine;
