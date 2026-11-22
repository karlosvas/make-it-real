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
