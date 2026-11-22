//! Registro de escenarios de cada ejercicio (el "arnés" que comprueba tu código).

pub mod common;
mod ex01_02;
mod ex03;
mod ex04_06;
mod ex07_10;

use crate::sim::runner::Scenario;

pub type Builder = fn() -> Scenario;

pub fn scenarios(exercise_id: &str) -> Option<Vec<(&'static str, Builder)>> {
    let list = match exercise_id {
        "01-ve-hacia-la-puerta" => ex01_02::scenarios_01(),
        "02-para-a-un-metro" => ex01_02::scenarios_02(),
        "03-solo-desde-delante" => ex03::scenarios(),
        "04-y-mirandola" => ex04_06::scenarios_04(),
        "05-abrete-suave" => ex04_06::scenarios_05(),
        "06-sin-llave-no-entras" => ex04_06::scenarios_06(),
        "07-la-llave-mas-cercana" => ex07_10::scenarios_07(),
        "08-patrulla" => ex07_10::scenarios_08(),
        "09-te-ve-el-guardia" => ex07_10::scenarios_09(),
        "10-la-puerta-se-cierra-sola" => ex07_10::scenarios_10(),
        _ => return None,
    };
    Some(list)
}

#[cfg(test)]
mod tests {
    use crate::exercise;
    use crate::run::{RunOutcome, run_exercise};
    use std::collections::HashMap;

    fn solution_files(id: &str) -> HashMap<String, String> {
        let root = exercise::root();
        let (meta, dir) = exercise::load(&root, id).unwrap();
        meta.files
            .iter()
            .filter(|f| f.editable)
            .map(|f| (f.name.clone(), std::fs::read_to_string(dir.join("solution").join(&f.name)).unwrap()))
            .collect()
    }

    /// Teletransportar al guardia con SetActorLocation recorre los waypoints "en orden", pero no vale.
    #[test]
    fn patrol_rejects_teleport() {
        let code = r#"#include "Guard.h"
AGuard::AGuard() { PrimaryActorTick.bCanEverTick = true; GetCharacterMovement()->MaxWalkSpeed = 300.f; }
void AGuard::Tick(float DeltaSeconds)
{
    Super::Tick(DeltaSeconds);
    if (Waypoints.Num() == 0) return;
    SetActorLocation(Waypoints[CurrentIndex]->GetActorLocation());
    CurrentIndex = (CurrentIndex + 1) % Waypoints.Num();
}
"#;
        let files = HashMap::from([("Guard.cpp".to_string(), code.to_string())]);
        match run_exercise(&exercise::root(), "08-patrulla", &files).unwrap() {
            RunOutcome::Ran { scenarios, .. } => {
                for s in scenarios.iter().filter(|s| s.id != "sin-puntos") {
                    assert!(!s.passed, "{}: el teletransporte no debería pasar", s.id);
                    assert!(s.message.contains("teletransportado"), "{}: mensaje inesperado: {}", s.id, s.message);
                }
            }
            RunOutcome::CompileError { log, .. } => panic!("no compila:\n{log}"),
        }
    }

    /// Puerta automática (10): cada error típico falla donde toca y con un mensaje que lo explica.
    /// `None` = ese escenario tiene que pasar; `Some(texto)` = tiene que fallar con un mensaje que contenga `texto`.
    #[test]
    fn auto_door_catches_common_mistakes() {
        const BLOCKED: &str = "bool ADoor::IsDoorwayBlocked() const { return Monigote && GetHorizontalDistanceTo(Monigote) < DoorwayRadius; }\n";
        let cases: &[(&str, &str, [Option<&str>; 4])] = &[
            (
                "el temporizador llama a Close() directamente",
                "void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::Close, AutoCloseDelay, false); }\n\
                 void ADoor::TryAutoClose() {}\n",
                [None, None, Some("encima del monigote"), Some("encima del monigote")],
            ),
            (
                "un FTimerHandle nuevo en cada Open()",
                "void ADoor::Open() { bIsOpen = true; FTimerHandle H; GetWorldTimerManager().SetTimer(H, this, &ADoor::TryAutoClose, AutoCloseDelay, false); }\n\
                 void ADoor::TryAutoClose() { if (IsDoorwayBlocked()) { FTimerHandle H; GetWorldTimerManager().SetTimer(H, this, &ADoor::TryAutoClose, RetryInterval, false); } else { Close(); } }\n",
                [None, Some("primer Open()"), None, Some("reintento pendiente")],
            ),
            (
                "no lo vuelve a intentar",
                "void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, AutoCloseDelay, false); }\n\
                 void ADoor::TryAutoClose() { if (!IsDoorwayBlocked()) { Close(); } }\n",
                [None, None, Some("abierta para siempre"), None],
            ),
            (
                "reintenta con AutoCloseDelay",
                "void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, AutoCloseDelay, false); }\n\
                 void ADoor::TryAutoClose() { if (IsDoorwayBlocked()) { GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, AutoCloseDelay, false); } else { Close(); } }\n",
                [None, None, Some("después de que se apartara"), None],
            ),
            (
                "reintento en otro handle que Open() no cancela",
                "static FTimerHandle RetryHandle;\n\
                 void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, AutoCloseDelay, false); }\n\
                 void ADoor::TryAutoClose() { if (IsDoorwayBlocked()) { GetWorldTimerManager().SetTimer(RetryHandle, this, &ADoor::TryAutoClose, RetryInterval, false); } else { Close(); } }\n",
                [None, None, None, Some("reintento pendiente")],
            ),
            (
                "temporizador en bucle sin ClearTimer",
                "void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, RetryInterval, true, AutoCloseDelay); }\n\
                 void ADoor::TryAutoClose() { if (!IsDoorwayBlocked()) { Close(); } }\n",
                [Some("ClearTimer"), Some("ClearTimer"), Some("ClearTimer"), Some("ClearTimer")],
            ),
            (
                "temporizador en bucle con ClearTimer (otra solución válida)",
                "void ADoor::Open() { bIsOpen = true; GetWorldTimerManager().SetTimer(CloseTimerHandle, this, &ADoor::TryAutoClose, RetryInterval, true, AutoCloseDelay); }\n\
                 void ADoor::TryAutoClose() { if (!IsDoorwayBlocked()) { GetWorldTimerManager().ClearTimer(CloseTimerHandle); Close(); } }\n",
                [None, None, None, None],
            ),
        ];
        let ids = ["se-cierra-sola", "reabrir", "alguien-en-medio", "reabrir-mientras-espera"];
        for (what, body, expected) in cases {
            let code = format!("#include \"Door.h\"\n{body}{BLOCKED}");
            let files = HashMap::from([("Door.cpp".to_string(), code)]);
            let RunOutcome::Ran { scenarios, .. } = run_exercise(&exercise::root(), "10-la-puerta-se-cierra-sola", &files).unwrap() else {
                panic!("{what}: no compila");
            };
            for (id, want) in ids.iter().zip(expected) {
                let s = scenarios.iter().find(|s| s.id == *id).unwrap();
                match want {
                    None => assert!(s.passed, "{what} / {id}: debería pasar, pero: {}", s.message),
                    Some(text) => assert!(!s.passed && s.message.contains(text), "{what} / {id}: se esperaba un fallo con «{text}», pero: {}", s.message),
                }
            }
        }
    }

    /// Para cada ejercicio: la solución pasa todos los escenarios y la plantilla compila pero falla alguno.
    #[test]
    fn every_exercise_solution_passes_and_template_fails() {
        let root = exercise::root();
        let all = exercise::list(&root);
        assert!(!all.is_empty(), "no hay ejercicios");
        for meta in all {
            let registry = super::scenarios(&meta.id).unwrap_or_else(|| panic!("{} no tiene escenarios registrados", meta.id));
            for s in &meta.scenarios {
                assert!(registry.iter().any(|(id, _)| *id == s.id), "{}: falta el escenario {}", meta.id, s.id);
            }
            assert_eq!(registry.len(), meta.scenarios.len(), "{}: escenarios de exercise.json y Rust no coinciden", meta.id);

            let solved = run_exercise(&root, &meta.id, &solution_files(&meta.id)).unwrap();
            match &solved {
                RunOutcome::Ran { scenarios, .. } => {
                    for s in scenarios {
                        assert!(s.passed, "{} / {}: la solución falla: {}", meta.id, s.id, s.message);
                    }
                }
                RunOutcome::CompileError { log, .. } => panic!("{}: la solución no compila:\n{log}", meta.id),
            }

            let template = run_exercise(&root, &meta.id, &HashMap::new()).unwrap();
            match &template {
                RunOutcome::Ran { scenarios, .. } => {
                    assert!(scenarios.iter().any(|s| !s.passed), "{}: la plantilla pasa todos los escenarios", meta.id);
                }
                RunOutcome::CompileError { log, .. } => panic!("{}: la plantilla no compila:\n{log}", meta.id),
            }
        }
    }
}
