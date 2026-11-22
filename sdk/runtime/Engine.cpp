// Implementación de AActor, componentes, mundo, timers y debug draw.
// Todo lo que toca el mundo pasa por imports ("ue.*") que implementa el servidor Rust.
#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "GameFramework/Character.h"
#include "DrawDebugHelpers.h"
#include "Mir/Glue.h"

MIR_IMPORT(get_location) void MirGetLocation(int32 Handle, double* Out);
MIR_IMPORT(set_location) void MirSetLocation(int32 Handle, double X, double Y, double Z, int32 bSweep);
MIR_IMPORT(get_rotation) void MirGetRotation(int32 Handle, double* Out);
MIR_IMPORT(set_rotation) void MirSetRotation(int32 Handle, double Pitch, double Yaw, double Roll);
MIR_IMPORT(get_velocity) void MirGetVelocity(int32 Handle, double* Out);
MIR_IMPORT(add_movement_input) void MirAddMovementInput(int32 Handle, double X, double Y, double Z, double Scale, double MaxWalkSpeed, int32 bOrient);
MIR_IMPORT(get_comp_rotation) void MirGetCompRotation(int32 Handle, int32 Index, double* Out);
MIR_IMPORT(set_comp_rotation) void MirSetCompRotation(int32 Handle, int32 Index, double Pitch, double Yaw, double Roll);
MIR_IMPORT(destroy) int32 MirDestroy(int32 Handle);
MIR_IMPORT(set_hidden) void MirSetHidden(int32 Handle, int32 bHidden);
MIR_IMPORT(is_hidden) int32 MirIsHidden(int32 Handle);
MIR_IMPORT(get_name) int32 MirGetName(int32 Handle, char* Buffer, int32 Capacity);
MIR_IMPORT(time) double MirTime();
MIR_IMPORT(delta) double MirDelta();
MIR_IMPORT(screen) void MirScreen(uint32 Color, double Duration, const char* Message, int32 Len);
MIR_IMPORT(draw) void MirDraw(int32 Kind, double AX, double AY, double AZ, double BX, double BY, double BZ, double Size, double Extra, uint32 Color, double LifeTime, int32 bPersistent);
MIR_IMPORT(draw_text) void MirDrawText(double X, double Y, double Z, const char* Text, int32 Len, uint32 Color, double LifeTime);

// ---------- Constantes ----------
const FVector FVector::ZeroVector(0, 0, 0);
const FVector FVector::OneVector(1, 1, 1);
const FVector FVector::UpVector(0, 0, 1);
const FVector FVector::DownVector(0, 0, -1);
const FVector FVector::ForwardVector(1, 0, 0);
const FVector FVector::BackwardVector(-1, 0, 0);
const FVector FVector::RightVector(0, 1, 0);
const FVector FVector::LeftVector(0, -1, 0);
const FVector FVector::XAxisVector(1, 0, 0);
const FVector FVector::YAxisVector(0, 1, 0);
const FVector FVector::ZAxisVector(0, 0, 1);
const FRotator FRotator::ZeroRotator(0, 0, 0);
const FColor FColor::White(255, 255, 255);
const FColor FColor::Black(0, 0, 0);
const FColor FColor::Transparent(0, 0, 0, 0);
const FColor FColor::Red(255, 0, 0);
const FColor FColor::Green(0, 255, 0);
const FColor FColor::Blue(0, 0, 255);
const FColor FColor::Yellow(255, 255, 0);
const FColor FColor::Cyan(0, 255, 255);
const FColor FColor::Magenta(255, 0, 255);
const FColor FColor::Orange(243, 156, 18);
const FColor FColor::Purple(169, 7, 228);
const FColor FColor::Turquoise(26, 188, 156);
const FColor FColor::Silver(189, 195, 199);
const FColor FColor::Emerald(46, 204, 113);
const FLinearColor FLinearColor::White(1, 1, 1);
const FLinearColor FLinearColor::Black(0, 0, 0);
const FLinearColor FLinearColor::Red(1, 0, 0);
const FLinearColor FLinearColor::Green(0, 1, 0);
const FLinearColor FLinearColor::Blue(0, 0, 1);
const FLinearColor FLinearColor::Yellow(1, 1, 0);

FString FVector::ToString() const { return FString::Printf("X=%.3f Y=%.3f Z=%.3f", X, Y, Z); }
FString FRotator::ToString() const { return FString::Printf("P=%f Y=%f R=%f", Pitch, Yaw, Roll); }

// ---------- Aleatorios deterministas ----------
static uint32 GRandSeed = 20261122u;
void FMath::RandInit(int32 Seed) { GRandSeed = (uint32)Seed; }
int32 FMath::Rand()
{
	GRandSeed = GRandSeed * 196314165u + 907633515u;
	return (int32)(GRandSeed >> 1);
}
float FMath::FRand() { return (float)((uint32)Rand() & 0xFFFFFF) / (float)0x1000000; }
int32 FMath::RandRange(int32 Min, int32 Max)
{
	const int32 Range = (Max - Min) + 1;
	return Min + (Range > 0 ? (int32)((uint32)Rand() % (uint32)Range) : 0);
}
float FMath::FRandRange(float Min, float Max) { return Min + (Max - Min) * FRand(); }
bool FMath::RandBool() { return (Rand() & 1) == 1; }

// ---------- Seguridad: llamadas sobre punteros nulos ----------
#define MIR_CHECK_SELF(What)                                                                                                  \
	if (this == nullptr)                                                                                                      \
	{                                                                                                                         \
		::Mir::Fatal("Access violation: has llamado a " What " sobre un puntero nulo (nullptr). En Unreal esto cerraria el editor."); \
	}

// ---------- AActor ----------
FVector AActor::GetActorLocation() const
{
	MIR_CHECK_SELF("GetActorLocation()");
	double V[3];
	MirGetLocation(MirHandle, V);
	return FVector(V[0], V[1], V[2]);
}

bool AActor::SetActorLocation(const FVector& NewLocation, bool bSweep)
{
	MIR_CHECK_SELF("SetActorLocation()");
	MirSetLocation(MirHandle, NewLocation.X, NewLocation.Y, NewLocation.Z, bSweep ? 1 : 0);
	return true;
}

FRotator AActor::GetActorRotation() const
{
	MIR_CHECK_SELF("GetActorRotation()");
	double V[3];
	MirGetRotation(MirHandle, V);
	return FRotator(V[0], V[1], V[2]);
}

bool AActor::SetActorRotation(const FRotator& NewRotation)
{
	MIR_CHECK_SELF("SetActorRotation()");
	MirSetRotation(MirHandle, NewRotation.Pitch, NewRotation.Yaw, NewRotation.Roll);
	return true;
}

FVector AActor::GetActorForwardVector() const { MIR_CHECK_SELF("GetActorForwardVector()"); return GetActorRotation().Vector(); }
FVector AActor::GetActorRightVector() const { MIR_CHECK_SELF("GetActorRightVector()"); return GetActorRotation().GetRightVector(); }
FVector AActor::GetActorUpVector() const { MIR_CHECK_SELF("GetActorUpVector()"); return GetActorRotation().GetUpVector(); }

double AActor::GetDistanceTo(const AActor* OtherActor) const
{
	MIR_CHECK_SELF("GetDistanceTo()");
	return OtherActor ? FVector::Dist(GetActorLocation(), OtherActor->GetActorLocation()) : 0.0;
}

double AActor::GetHorizontalDistanceTo(const AActor* OtherActor) const
{
	MIR_CHECK_SELF("GetHorizontalDistanceTo()");
	return OtherActor ? FVector::Dist2D(GetActorLocation(), OtherActor->GetActorLocation()) : 0.0;
}

FVector AActor::GetVelocity() const
{
	MIR_CHECK_SELF("GetVelocity()");
	double V[3];
	MirGetVelocity(MirHandle, V);
	return FVector(V[0], V[1], V[2]);
}

bool AActor::Destroy()
{
	MIR_CHECK_SELF("Destroy()");
	return MirDestroy(MirHandle) != 0;
}

bool AActor::IsHidden() const { MIR_CHECK_SELF("IsHidden()"); return MirIsHidden(MirHandle) != 0; }
void AActor::SetActorHiddenInGame(bool bNewHidden) { MIR_CHECK_SELF("SetActorHiddenInGame()"); MirSetHidden(MirHandle, bNewHidden ? 1 : 0); }

FString AActor::GetName() const
{
	MIR_CHECK_SELF("GetName()");
	char Buffer[128];
	const int32 Len = MirGetName(MirHandle, Buffer, sizeof(Buffer) - 1);
	Buffer[Len < 0 ? 0 : (Len > 127 ? 127 : Len)] = 0;
	return FString(Buffer);
}

static UWorld GMirWorld;
static FTimerManager GMirTimerManager;

UWorld* AActor::GetWorld() const { return &GMirWorld; }
FTimerManager& AActor::GetWorldTimerManager() const { return GMirTimerManager; }

double UWorld::GetTimeSeconds() const { return MirTime(); }
float UWorld::GetDeltaSeconds() const { return (float)MirDelta(); }
FTimerManager& UWorld::GetTimerManager() const { return GMirTimerManager; }

// ---------- Componentes ----------
void USceneComponent::SetRelativeRotation(const FRotator& NewRotation)
{
	MIR_CHECK_SELF("SetRelativeRotation()");
	MirSetCompRotation(MirOwner ? MirOwner->MirHandle : -1, MirIndex, NewRotation.Pitch, NewRotation.Yaw, NewRotation.Roll);
}

FRotator USceneComponent::GetRelativeRotation() const
{
	MIR_CHECK_SELF("GetRelativeRotation()");
	double V[3];
	MirGetCompRotation(MirOwner ? MirOwner->MirHandle : -1, MirIndex, V);
	return FRotator(V[0], V[1], V[2]);
}

FVector USceneComponent::GetComponentLocation() const
{
	MIR_CHECK_SELF("GetComponentLocation()");
	return MirOwner ? MirOwner->GetActorLocation() : FVector::ZeroVector;
}

// ---------- Pawn / Character ----------
void APawn::AddMovementInput(FVector WorldDirection, float ScaleValue, bool /*bForce*/)
{
	MIR_CHECK_SELF("AddMovementInput()");
	MirAddMovementInput(MirHandle, WorldDirection.X, WorldDirection.Y, WorldDirection.Z, ScaleValue, 400.0, 1);
}

void ACharacter::AddMovementInput(FVector WorldDirection, float ScaleValue, bool /*bForce*/)
{
	MIR_CHECK_SELF("AddMovementInput()");
	MirAddMovementInput(MirHandle, WorldDirection.X, WorldDirection.Y, WorldDirection.Z, ScaleValue, CharacterMovement.MaxWalkSpeed, CharacterMovement.bOrientRotationToMovement ? 1 : 0);
}

FVector APawn::GetVelocity() const { return AActor::GetVelocity(); }

// ---------- Timers ----------
namespace
{
	struct FMirTimer
	{
		uint64 Id = 0;
		FMirTimerCallback* Callback = nullptr;
		float Rate = 0.f;
		float Remaining = 0.f;
		float Elapsed = 0.f;
		bool bLoop = false;
		bool bActive = false;
	};
	constexpr int32 MaxTimers = 64;
	FMirTimer GTimers[MaxTimers];
	uint64 GNextTimerId = 1;

	FMirTimer* FindTimer(uint64 Id)
	{
		if (Id == 0) { return nullptr; }
		for (FMirTimer& Timer : GTimers) { if (Timer.bActive && Timer.Id == Id) { return &Timer; } }
		return nullptr;
	}
}

void FTimerManager::SetTimerInternal(FTimerHandle& InOutHandle, FMirTimerCallback* Callback, float InRate, bool bLoop, float InFirstDelay)
{
	ClearTimer(InOutHandle);
	if (InRate <= 0.f) { return; }
	for (FMirTimer& Timer : GTimers)
	{
		if (!Timer.bActive)
		{
			Timer.Id = GNextTimerId++;
			Timer.Callback = Callback;
			Timer.Rate = InRate;
			Timer.Remaining = InFirstDelay >= 0.f ? InFirstDelay : InRate;
			Timer.Elapsed = 0.f;
			Timer.bLoop = bLoop;
			Timer.bActive = true;
			InOutHandle.Handle = Timer.Id;
			return;
		}
	}
	::Mir::Fatal("Demasiados temporizadores activos a la vez (maximo 64)");
}

void FTimerManager::ClearTimer(FTimerHandle& InHandle)
{
	if (FMirTimer* Timer = FindTimer(InHandle.Handle)) { Timer->bActive = false; }
	InHandle.Invalidate();
}

bool FTimerManager::IsTimerActive(FTimerHandle InHandle) const { return FindTimer(InHandle.Handle) != nullptr; }
float FTimerManager::GetTimerRemaining(FTimerHandle InHandle) const
{
	const FMirTimer* Timer = FindTimer(InHandle.Handle);
	return Timer ? Timer->Remaining : -1.f;
}
float FTimerManager::GetTimerElapsed(FTimerHandle InHandle) const
{
	const FMirTimer* Timer = FindTimer(InHandle.Handle);
	return Timer ? Timer->Elapsed : -1.f;
}
float FTimerManager::GetTimerRate(FTimerHandle InHandle) const
{
	const FMirTimer* Timer = FindTimer(InHandle.Handle);
	return Timer ? Timer->Rate : -1.f;
}

void FTimerManager::MirTick(float DeltaTime)
{
	for (int32 i = 0; i < MaxTimers; ++i)
	{
		FMirTimer& Timer = GTimers[i];
		if (!Timer.bActive) { continue; }
		Timer.Remaining -= DeltaTime;
		Timer.Elapsed += DeltaTime;
		if (Timer.Remaining <= 0.f)
		{
			FMirTimerCallback* Callback = Timer.Callback;
			if (Timer.bLoop) { Timer.Remaining += Timer.Rate; Timer.Elapsed = 0.f; }
			else { Timer.bActive = false; }
			Callback->Execute();
		}
	}
}

// ---------- GEngine ----------
static UEngine GMirEngine;
UEngine* GEngine = &GMirEngine;

void UEngine::AddOnScreenDebugMessage(int32 /*Key*/, float TimeToDisplay, FColor DisplayColor, const FString& DebugMessage, bool /*bNewerOnTop*/, const FVector2D& /*TextScale*/)
{
	MirScreen(DisplayColor.ToPackedRGBA(), TimeToDisplay, *DebugMessage, DebugMessage.Len());
}

// ---------- Debug draw ----------
enum EMirDraw : int32 { Draw_Line = 0, Draw_Point = 1, Draw_Arrow = 2, Draw_Sphere = 3, Draw_Circle = 4, Draw_Cone = 5 };

void DrawDebugLine(const UWorld*, FVector const& A, FVector const& B, FColor const& Color, bool bPersistent, float LifeTime, uint8, float Thickness)
{
	MirDraw(Draw_Line, A.X, A.Y, A.Z, B.X, B.Y, B.Z, Thickness, 0, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugPoint(const UWorld*, FVector const& P, float Size, FColor const& Color, bool bPersistent, float LifeTime, uint8)
{
	MirDraw(Draw_Point, P.X, P.Y, P.Z, 0, 0, 0, Size, 0, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugDirectionalArrow(const UWorld*, FVector const& A, FVector const& B, float ArrowSize, FColor const& Color, bool bPersistent, float LifeTime, uint8, float)
{
	MirDraw(Draw_Arrow, A.X, A.Y, A.Z, B.X, B.Y, B.Z, ArrowSize, 0, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugSphere(const UWorld*, FVector const& C, float Radius, int32, FColor const& Color, bool bPersistent, float LifeTime, uint8, float)
{
	MirDraw(Draw_Sphere, C.X, C.Y, C.Z, 0, 0, 0, Radius, 0, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugCircle(const UWorld*, FVector const& C, float Radius, int32, FColor const& Color, bool bPersistent, float LifeTime, uint8, float)
{
	MirDraw(Draw_Circle, C.X, C.Y, C.Z, 0, 0, 1, Radius, 0, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugCone(const UWorld*, FVector const& Origin, FVector const& Direction, float Length, float AngleWidth, float, int32, FColor const& Color, bool bPersistent, float LifeTime, uint8, float)
{
	const FVector Dir = Direction.GetSafeNormal();
	MirDraw(Draw_Cone, Origin.X, Origin.Y, Origin.Z, Dir.X, Dir.Y, Dir.Z, Length, AngleWidth, Color.ToPackedRGBA(), LifeTime, bPersistent);
}
void DrawDebugString(const UWorld*, FVector const& Location, const FString& Text, AActor* BaseActor, FColor const& Color, float Duration, bool, float)
{
	const FVector P = BaseActor ? BaseActor->GetActorLocation() + Location : Location;
	MirDrawText(P.X, P.Y, P.Z, *Text, Text.Len(), Color.ToPackedRGBA(), Duration);
}

// ---------- Registro de actores y exports para el host ----------
static AActor* GMirActors[256];

AActor* MirActor(int32 Handle)
{
	if (Handle < 0 || Handle >= 256) { return nullptr; }
	return GMirActors[Handle];
}

MIR_EXPORT(mir_create) int32 mir_create(int32 ClassId, int32 Handle)
{
	if (Handle < 0 || Handle >= 256) { return 0; }
	AActor* Actor = MirCreateActor(ClassId);
	if (!Actor) { return 0; }
	Actor->MirHandle = Handle;
	GMirActors[Handle] = Actor;
	return 1;
}

MIR_EXPORT(mir_wire) void mir_wire(int32 Slot, int32 A, int32 B) { MirWire(Slot, A, B); }
MIR_EXPORT(mir_call) int32 mir_call(int32 Function, int32 A, int32 B) { return MirCall(Function, A, B); }

MIR_EXPORT(mir_begin_play) void mir_begin_play(int32 Handle)
{
	if (AActor* Actor = MirActor(Handle)) { Actor->BeginPlay(); }
}

MIR_EXPORT(mir_tick) void mir_tick(int32 Handle, float DeltaSeconds)
{
	AActor* Actor = MirActor(Handle);
	if (Actor && Actor->PrimaryActorTick.bCanEverTick && Actor->PrimaryActorTick.bStartWithTickEnabled) { Actor->Tick(DeltaSeconds); }
}

MIR_EXPORT(mir_tick_timers) void mir_tick_timers(float DeltaSeconds) { GMirTimerManager.MirTick(DeltaSeconds); }
