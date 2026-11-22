// Catálogo de la API del mini-Unreal: alimenta el autocompletado de Monaco y la chuleta.
export interface ApiEntry {
  /** Nombre completo, p. ej. "FVector::DotProduct" */
  id: string;
  /** Lo que se escribe/inserta */
  label: string;
  kind: 'function' | 'method' | 'class' | 'field' | 'macro' | 'constant';
  signature: string;
  doc: string;
  /** Snippet de Monaco */
  insert?: string;
}

export const UE_API: ApiEntry[] = [
  // FVector
  { id: 'FVector', label: 'FVector', kind: 'class', signature: 'struct FVector { double X, Y, Z; }', doc: 'Vector 3D en centímetros. X = delante, Y = derecha, Z = arriba.' },
  { id: 'FVector::DotProduct', label: 'DotProduct', kind: 'function', signature: 'static double FVector::DotProduct(const FVector& A, const FVector& B)', doc: 'Producto escalar. Con vectores unitarios: 1 = misma dirección, 0 = perpendiculares, -1 = opuestos. También A | B.', insert: 'DotProduct(${1:A}, ${2:B})' },
  { id: 'FVector::CrossProduct', label: 'CrossProduct', kind: 'function', signature: 'static FVector FVector::CrossProduct(const FVector& A, const FVector& B)', doc: 'Producto vectorial: un vector perpendicular a A y a B. También A ^ B.', insert: 'CrossProduct(${1:A}, ${2:B})' },
  { id: 'FVector::Dist', label: 'Dist', kind: 'function', signature: 'static double FVector::Dist(const FVector& A, const FVector& B)', doc: 'Distancia entre dos puntos (cm).', insert: 'Dist(${1:A}, ${2:B})' },
  { id: 'FVector::DistSquared', label: 'DistSquared', kind: 'function', signature: 'static double FVector::DistSquared(const FVector& A, const FVector& B)', doc: 'Distancia al cuadrado: más barata que Dist, perfecta para comparar cuál está más cerca.', insert: 'DistSquared(${1:A}, ${2:B})' },
  { id: 'FVector::Dist2D', label: 'Dist2D', kind: 'function', signature: 'static double FVector::Dist2D(const FVector& A, const FVector& B)', doc: 'Distancia ignorando la altura (Z).', insert: 'Dist2D(${1:A}, ${2:B})' },
  { id: 'FVector::Size', label: 'Size', kind: 'method', signature: 'double FVector::Size() const', doc: 'Longitud del vector.', insert: 'Size()' },
  { id: 'FVector::Size2D', label: 'Size2D', kind: 'method', signature: 'double FVector::Size2D() const', doc: 'Longitud ignorando Z.', insert: 'Size2D()' },
  { id: 'FVector::SizeSquared', label: 'SizeSquared', kind: 'method', signature: 'double FVector::SizeSquared() const', doc: 'Longitud al cuadrado.', insert: 'SizeSquared()' },
  { id: 'FVector::GetSafeNormal', label: 'GetSafeNormal', kind: 'method', signature: 'FVector FVector::GetSafeNormal(double Tolerance = SMALL_NUMBER) const', doc: 'Copia del vector con longitud 1 (o ZeroVector si es casi cero).', insert: 'GetSafeNormal()' },
  { id: 'FVector::GetSafeNormal2D', label: 'GetSafeNormal2D', kind: 'method', signature: 'FVector FVector::GetSafeNormal2D() const', doc: 'Como GetSafeNormal pero con Z = 0.', insert: 'GetSafeNormal2D()' },
  { id: 'FVector::Normalize', label: 'Normalize', kind: 'method', signature: 'bool FVector::Normalize(double Tolerance = SMALL_NUMBER)', doc: 'Normaliza el propio vector. Devuelve false si era casi cero.', insert: 'Normalize()' },
  { id: 'FVector::IsNearlyZero', label: 'IsNearlyZero', kind: 'method', signature: 'bool FVector::IsNearlyZero(double Tolerance = KINDA_SMALL_NUMBER) const', doc: 'true si todos sus componentes son casi 0.', insert: 'IsNearlyZero()' },
  { id: 'FVector::Rotation', label: 'Rotation', kind: 'method', signature: 'FRotator FVector::Rotation() const', doc: 'Rotación que apunta en la dirección del vector.', insert: 'Rotation()' },
  { id: 'FVector::ToString', label: 'ToString', kind: 'method', signature: 'FString FVector::ToString() const', doc: 'Texto "X=… Y=… Z=…" para usar con UE_LOG: *V.ToString()', insert: 'ToString()' },
  { id: 'FVector::ZeroVector', label: 'ZeroVector', kind: 'constant', signature: 'static const FVector FVector::ZeroVector', doc: '(0, 0, 0)' },
  { id: 'FVector::ForwardVector', label: 'ForwardVector', kind: 'constant', signature: 'static const FVector FVector::ForwardVector', doc: '(1, 0, 0)' },
  { id: 'FVector::UpVector', label: 'UpVector', kind: 'constant', signature: 'static const FVector FVector::UpVector', doc: '(0, 0, 1)' },
  { id: 'FVector::RightVector', label: 'RightVector', kind: 'constant', signature: 'static const FVector FVector::RightVector', doc: '(0, 1, 0)' },

  // FRotator
  { id: 'FRotator', label: 'FRotator', kind: 'class', signature: 'struct FRotator { double Pitch, Yaw, Roll; }', doc: 'Rotación en grados. Yaw = girar a izquierda/derecha.', insert: 'FRotator(${1:0.f}, ${2:Yaw}, ${3:0.f})' },
  { id: 'FRotator::Vector', label: 'Vector', kind: 'method', signature: 'FVector FRotator::Vector() const', doc: 'Vector unitario hacia donde apunta la rotación.', insert: 'Vector()' },
  { id: 'FRotator::RotateVector', label: 'RotateVector', kind: 'method', signature: 'FVector FRotator::RotateVector(const FVector& V) const', doc: 'Rota V con esta rotación.', insert: 'RotateVector(${1:V})' },
  { id: 'FRotator::ZeroRotator', label: 'ZeroRotator', kind: 'constant', signature: 'static const FRotator FRotator::ZeroRotator', doc: '(0, 0, 0)' },

  // FMath
  { id: 'FMath', label: 'FMath', kind: 'class', signature: 'struct FMath', doc: 'Funciones matemáticas de Unreal.' },
  { id: 'FMath::Abs', label: 'Abs', kind: 'function', signature: 'T FMath::Abs(T A)', doc: 'Valor absoluto.', insert: 'Abs(${1:A})' },
  { id: 'FMath::Min', label: 'Min', kind: 'function', signature: 'T FMath::Min(T A, T B)', doc: 'El menor de dos valores.', insert: 'Min(${1:A}, ${2:B})' },
  { id: 'FMath::Max', label: 'Max', kind: 'function', signature: 'T FMath::Max(T A, T B)', doc: 'El mayor de dos valores.', insert: 'Max(${1:A}, ${2:B})' },
  { id: 'FMath::Clamp', label: 'Clamp', kind: 'function', signature: 'T FMath::Clamp(T X, T Min, T Max)', doc: 'Limita X al rango [Min, Max].', insert: 'Clamp(${1:X}, ${2:Min}, ${3:Max})' },
  { id: 'FMath::Sqrt', label: 'Sqrt', kind: 'function', signature: 'double FMath::Sqrt(double X)', doc: 'Raíz cuadrada.', insert: 'Sqrt(${1:X})' },
  { id: 'FMath::Cos', label: 'Cos', kind: 'function', signature: 'double FMath::Cos(double Radians)', doc: 'Coseno (en radianes).', insert: 'Cos(${1:Radians})' },
  { id: 'FMath::Sin', label: 'Sin', kind: 'function', signature: 'double FMath::Sin(double Radians)', doc: 'Seno (en radianes).', insert: 'Sin(${1:Radians})' },
  { id: 'FMath::Acos', label: 'Acos', kind: 'function', signature: 'double FMath::Acos(double X)', doc: 'Arcocoseno: de un producto escalar (unitario) al ángulo en radianes.', insert: 'Acos(${1:X})' },
  { id: 'FMath::Atan2', label: 'Atan2', kind: 'function', signature: 'double FMath::Atan2(double Y, double X)', doc: 'Ángulo (radianes) del vector (X, Y).', insert: 'Atan2(${1:Y}, ${2:X})' },
  { id: 'FMath::DegreesToRadians', label: 'DegreesToRadians', kind: 'function', signature: 'double FMath::DegreesToRadians(double Degrees)', doc: 'Grados → radianes.', insert: 'DegreesToRadians(${1:Degrees})' },
  { id: 'FMath::RadiansToDegrees', label: 'RadiansToDegrees', kind: 'function', signature: 'double FMath::RadiansToDegrees(double Radians)', doc: 'Radianes → grados.', insert: 'RadiansToDegrees(${1:Radians})' },
  { id: 'FMath::Lerp', label: 'Lerp', kind: 'function', signature: 'T FMath::Lerp(T A, T B, float Alpha)', doc: 'Interpolación lineal: A + Alpha * (B - A).', insert: 'Lerp(${1:A}, ${2:B}, ${3:Alpha})' },
  { id: 'FMath::FInterpTo', label: 'FInterpTo', kind: 'function', signature: 'float FMath::FInterpTo(float Current, float Target, float DeltaTime, float InterpSpeed)', doc: 'Acerca Current a Target suavemente (rápido al principio, lento al final). Llámalo en cada Tick.', insert: 'FInterpTo(${1:Current}, ${2:Target}, ${3:DeltaSeconds}, ${4:InterpSpeed})' },
  { id: 'FMath::FInterpConstantTo', label: 'FInterpConstantTo', kind: 'function', signature: 'float FMath::FInterpConstantTo(float Current, float Target, float DeltaTime, float InterpSpeed)', doc: 'Acerca Current a Target a velocidad constante (InterpSpeed unidades por segundo).', insert: 'FInterpConstantTo(${1:Current}, ${2:Target}, ${3:DeltaSeconds}, ${4:Speed})' },
  { id: 'FMath::VInterpTo', label: 'VInterpTo', kind: 'function', signature: 'FVector FMath::VInterpTo(const FVector& Current, const FVector& Target, float DeltaTime, float InterpSpeed)', doc: 'FInterpTo para vectores.', insert: 'VInterpTo(${1:Current}, ${2:Target}, ${3:DeltaSeconds}, ${4:InterpSpeed})' },
  { id: 'FMath::RInterpTo', label: 'RInterpTo', kind: 'function', signature: 'FRotator FMath::RInterpTo(const FRotator& Current, const FRotator& Target, float DeltaTime, float InterpSpeed)', doc: 'FInterpTo para rotaciones.', insert: 'RInterpTo(${1:Current}, ${2:Target}, ${3:DeltaSeconds}, ${4:InterpSpeed})' },
  { id: 'FMath::IsNearlyEqual', label: 'IsNearlyEqual', kind: 'function', signature: 'bool FMath::IsNearlyEqual(double A, double B, double Tolerance = SMALL_NUMBER)', doc: 'Compara floats con tolerancia.', insert: 'IsNearlyEqual(${1:A}, ${2:B}, ${3:0.01})' },

  // AActor
  { id: 'AActor::GetActorLocation', label: 'GetActorLocation', kind: 'method', signature: 'FVector AActor::GetActorLocation() const', doc: 'Posición del actor en el mundo (cm).', insert: 'GetActorLocation()' },
  { id: 'AActor::SetActorLocation', label: 'SetActorLocation', kind: 'method', signature: 'bool AActor::SetActorLocation(const FVector& NewLocation)', doc: 'Teletransporta el actor.', insert: 'SetActorLocation(${1:NewLocation})' },
  { id: 'AActor::GetActorRotation', label: 'GetActorRotation', kind: 'method', signature: 'FRotator AActor::GetActorRotation() const', doc: 'Rotación del actor.', insert: 'GetActorRotation()' },
  { id: 'AActor::SetActorRotation', label: 'SetActorRotation', kind: 'method', signature: 'bool AActor::SetActorRotation(const FRotator& NewRotation)', doc: 'Cambia la rotación del actor.', insert: 'SetActorRotation(${1:NewRotation})' },
  { id: 'AActor::GetActorForwardVector', label: 'GetActorForwardVector', kind: 'method', signature: 'FVector AActor::GetActorForwardVector() const', doc: 'Vector unitario hacia el DELANTE del actor (su eje X).', insert: 'GetActorForwardVector()' },
  { id: 'AActor::GetActorRightVector', label: 'GetActorRightVector', kind: 'method', signature: 'FVector AActor::GetActorRightVector() const', doc: 'Vector unitario hacia la derecha del actor (su eje Y).', insert: 'GetActorRightVector()' },
  { id: 'AActor::GetDistanceTo', label: 'GetDistanceTo', kind: 'method', signature: 'double AActor::GetDistanceTo(const AActor* OtherActor) const', doc: 'Distancia hasta otro actor (cm).', insert: 'GetDistanceTo(${1:OtherActor})' },
  { id: 'AActor::GetVelocity', label: 'GetVelocity', kind: 'method', signature: 'FVector AActor::GetVelocity() const', doc: 'Velocidad actual (cm/s).', insert: 'GetVelocity()' },
  { id: 'AActor::Tick', label: 'Tick', kind: 'method', signature: 'virtual void AActor::Tick(float DeltaSeconds)', doc: 'Se llama en cada frame. DeltaSeconds = segundos desde el frame anterior (~0.0167).' },
  { id: 'AActor::BeginPlay', label: 'BeginPlay', kind: 'method', signature: 'virtual void AActor::BeginPlay()', doc: 'Se llama una vez al empezar la partida.' },
  { id: 'AActor::GetWorld', label: 'GetWorld', kind: 'method', signature: 'UWorld* AActor::GetWorld() const', doc: 'El mundo: tiempo, timers…', insert: 'GetWorld()' },
  { id: 'AActor::GetWorldTimerManager', label: 'GetWorldTimerManager', kind: 'method', signature: 'FTimerManager& AActor::GetWorldTimerManager() const', doc: 'El gestor de temporizadores.', insert: 'GetWorldTimerManager()' },
  { id: 'AActor::GetName', label: 'GetName', kind: 'method', signature: 'FString AActor::GetName() const', doc: 'Nombre del actor.', insert: 'GetName()' },
  { id: 'APawn::AddMovementInput', label: 'AddMovementInput', kind: 'method', signature: 'void APawn::AddMovementInput(FVector WorldDirection, float ScaleValue = 1.f)', doc: 'Pide moverse en esa dirección este frame. El componente de movimiento aplica MaxWalkSpeed (400 cm/s).', insert: 'AddMovementInput(${1:Direction})' },
  { id: 'ACharacter::GetCharacterMovement', label: 'GetCharacterMovement', kind: 'method', signature: 'UCharacterMovementComponent* ACharacter::GetCharacterMovement() const', doc: 'Componente de movimiento: ->MaxWalkSpeed, ->bOrientRotationToMovement.', insert: 'GetCharacterMovement()' },
  { id: 'USceneComponent::SetRelativeRotation', label: 'SetRelativeRotation', kind: 'method', signature: 'void USceneComponent::SetRelativeRotation(const FRotator& NewRotation)', doc: 'Rota el componente respecto a su actor (p. ej. la hoja de la puerta).', insert: 'SetRelativeRotation(${1:NewRotation})' },

  // Timers
  { id: 'FTimerManager::SetTimer', label: 'SetTimer', kind: 'method', signature: 'void FTimerManager::SetTimer(FTimerHandle& Handle, UserClass* Obj, void (UserClass::*Method)(), float Rate, bool bLoop = false)', doc: 'Llama a Method dentro de Rate segundos. Si el Handle ya tenía un timer, lo reemplaza.', insert: 'SetTimer(${1:Handle}, this, &${2:AMyActor}::${3:Method}, ${4:Rate}, ${5:false})' },
  { id: 'FTimerManager::ClearTimer', label: 'ClearTimer', kind: 'method', signature: 'void FTimerManager::ClearTimer(FTimerHandle& Handle)', doc: 'Cancela el temporizador.', insert: 'ClearTimer(${1:Handle})' },
  { id: 'FTimerManager::IsTimerActive', label: 'IsTimerActive', kind: 'method', signature: 'bool FTimerManager::IsTimerActive(FTimerHandle Handle) const', doc: 'true si el temporizador sigue en marcha.', insert: 'IsTimerActive(${1:Handle})' },
  { id: 'FTimerHandle', label: 'FTimerHandle', kind: 'class', signature: 'struct FTimerHandle', doc: 'Identifica un temporizador.' },
  { id: 'UWorld::GetTimeSeconds', label: 'GetTimeSeconds', kind: 'method', signature: 'double UWorld::GetTimeSeconds() const', doc: 'Segundos desde el inicio de la partida.', insert: 'GetTimeSeconds()' },

  // TArray
  { id: 'TArray', label: 'TArray', kind: 'class', signature: 'template<class T> class TArray', doc: 'Array dinámico de Unreal. Recorre con for (T Item : Array).' },
  { id: 'TArray::Num', label: 'Num', kind: 'method', signature: 'int32 TArray::Num() const', doc: 'Número de elementos.', insert: 'Num()' },
  { id: 'TArray::IsEmpty', label: 'IsEmpty', kind: 'method', signature: 'bool TArray::IsEmpty() const', doc: 'true si no tiene elementos.', insert: 'IsEmpty()' },
  { id: 'TArray::Add', label: 'Add', kind: 'method', signature: 'int32 TArray::Add(const T& Item)', doc: 'Añade al final.', insert: 'Add(${1:Item})' },
  { id: 'TArray::IsValidIndex', label: 'IsValidIndex', kind: 'method', signature: 'bool TArray::IsValidIndex(int32 Index) const', doc: 'true si Index está dentro del array.', insert: 'IsValidIndex(${1:Index})' },
  { id: 'TArray::Contains', label: 'Contains', kind: 'method', signature: 'bool TArray::Contains(const T& Item) const', doc: 'true si contiene Item.', insert: 'Contains(${1:Item})' },

  // Logs y debug
  { id: 'UE_LOG', label: 'UE_LOG', kind: 'macro', signature: 'UE_LOG(LogTemp, Warning, TEXT("Formato %f"), Valor)', doc: 'Escribe en el Output Log. Verbosidades: Log, Display, Warning, Error.', insert: 'UE_LOG(LogTemp, ${1|Log,Warning,Error,Display|}, TEXT("${2:Mensaje}")${3});' },
  { id: 'TEXT', label: 'TEXT', kind: 'macro', signature: 'TEXT("...")', doc: 'Literal de texto de Unreal.', insert: 'TEXT("${1}")' },
  { id: 'GEngine->AddOnScreenDebugMessage', label: 'AddOnScreenDebugMessage', kind: 'method', signature: 'void UEngine::AddOnScreenDebugMessage(int32 Key, float Time, FColor Color, const FString& Msg)', doc: 'Pinta un mensaje encima del visor.', insert: 'AddOnScreenDebugMessage(-1, ${1:2.f}, FColor::${2:Yellow}, ${3:TEXT("Hola")});' },
  { id: 'DrawDebugLine', label: 'DrawDebugLine', kind: 'function', signature: 'void DrawDebugLine(const UWorld* World, FVector Start, FVector End, FColor Color, bool bPersistent = false, float LifeTime = -1.f)', doc: 'Dibuja una línea en el visor (incluye "DrawDebugHelpers.h").', insert: 'DrawDebugLine(GetWorld(), ${1:Start}, ${2:End}, FColor::${3:Green});' },
  { id: 'DrawDebugSphere', label: 'DrawDebugSphere', kind: 'function', signature: 'void DrawDebugSphere(const UWorld* World, FVector Center, float Radius, int32 Segments, FColor Color, bool bPersistent = false, float LifeTime = -1.f)', doc: 'Dibuja una esfera de alambre.', insert: 'DrawDebugSphere(GetWorld(), ${1:Center}, ${2:50.f}, 12, FColor::${3:Red});' },
  { id: 'DrawDebugDirectionalArrow', label: 'DrawDebugDirectionalArrow', kind: 'function', signature: 'void DrawDebugDirectionalArrow(const UWorld* World, FVector Start, FVector End, float ArrowSize, FColor Color)', doc: 'Dibuja una flecha.', insert: 'DrawDebugDirectionalArrow(GetWorld(), ${1:Start}, ${2:End}, 40.f, FColor::${3:Cyan});' },
  { id: 'FString::Printf', label: 'Printf', kind: 'function', signature: 'static FString FString::Printf(const TCHAR* Format, ...)', doc: 'Crea un FString con formato tipo printf.', insert: 'Printf(TEXT("${1}"), ${2})' },
  { id: 'nullptr', label: 'nullptr', kind: 'constant', signature: 'nullptr', doc: 'Puntero nulo. Comprueba siempre los punteros antes de usarlos: if (Ptr) { … }' },
];

export const API_BY_ID = new Map(UE_API.map((e) => [e.id, e]));
