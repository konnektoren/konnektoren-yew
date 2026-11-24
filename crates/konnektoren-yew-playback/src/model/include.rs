use super::{Excursion, PlaybackStep};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum IncludeError {
    #[error("Missing included excursion: {0}")]
    Missing(String),
    #[error("Circular excursion include: {0}")]
    Cycle(String),
    #[error("Included excursion {requested} has id {actual}")]
    IdMismatch { requested: String, actual: String },
    #[error("Included excursion {id} has version {actual}, expected {expected}")]
    VersionMismatch {
        id: String,
        expected: u32,
        actual: u32,
    },
}

impl Excursion {
    /// Expand includes in order. Only the root excursion's repeat setting applies.
    /// The caller loads the referenced excursions and decides which are available.
    pub fn resolve_includes(
        &self,
        excursions: &HashMap<String, Excursion>,
    ) -> Result<Self, IncludeError> {
        fn expand(
            tour: &Excursion,
            excursions: &HashMap<String, Excursion>,
            stack: &mut Vec<String>,
            steps: &mut Vec<PlaybackStep>,
        ) -> Result<(), IncludeError> {
            stack.push(tour.id.clone());
            for step in &tour.steps {
                if let PlaybackStep::Include { include } = step {
                    if stack.contains(include) {
                        return Err(IncludeError::Cycle(include.clone()));
                    }
                    let child = excursions
                        .get(include)
                        .ok_or_else(|| IncludeError::Missing(include.clone()))?;
                    if child.id != *include {
                        return Err(IncludeError::IdMismatch {
                            requested: include.clone(),
                            actual: child.id.clone(),
                        });
                    }
                    if child.version != tour.version {
                        return Err(IncludeError::VersionMismatch {
                            id: include.clone(),
                            expected: tour.version,
                            actual: child.version,
                        });
                    }
                    expand(child, excursions, stack, steps)?;
                } else {
                    steps.push(step.clone());
                }
            }
            stack.pop();
            Ok(())
        }

        let mut resolved = self.clone();
        resolved.steps.clear();
        expand(self, excursions, &mut Vec::new(), &mut resolved.steps)?;
        Ok(resolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn excursion(id: &str, steps: Vec<PlaybackStep>) -> Excursion {
        Excursion {
            version: 1,
            id: id.into(),
            title: id.into(),
            repeat: false,
            cursor: None,
            steps,
        }
    }
    fn say(value: &str) -> PlaybackStep {
        PlaybackStep::Say { say: value.into() }
    }
    fn include(value: &str) -> PlaybackStep {
        PlaybackStep::Include {
            include: value.into(),
        }
    }

    #[test]
    fn expands_nested_and_repeated_includes_in_order() {
        let mut root = excursion(
            "root",
            vec![
                say("before"),
                include("child"),
                include("child"),
                say("after"),
            ],
        );
        root.cursor = Some(super::super::CursorConfig {
            image: "root.svg".into(),
        });
        let mut child = excursion("child", vec![say("child"), include("leaf")]);
        child.repeat = true;
        child.cursor = Some(super::super::CursorConfig {
            image: "child.svg".into(),
        });
        let tours = HashMap::from([
            ("child".into(), child),
            ("leaf".into(), excursion("leaf", vec![say("leaf")])),
        ]);
        let result = root.resolve_includes(&tours).expect("valid includes");
        assert_eq!(
            result.steps,
            vec![
                say("before"),
                say("child"),
                say("leaf"),
                say("child"),
                say("leaf"),
                say("after")
            ]
        );
        assert!(!result.repeat);
        assert_eq!(result.cursor, root.cursor);
    }

    #[test]
    fn rejects_missing_cycles_mismatched_ids_and_versions() {
        let root = excursion("root", vec![include("child")]);
        assert_eq!(
            root.resolve_includes(&HashMap::new()),
            Err(IncludeError::Missing("child".into()))
        );
        let cycle = HashMap::from([("child".into(), excursion("child", vec![include("root")]))]);
        assert_eq!(
            root.resolve_includes(&cycle),
            Err(IncludeError::Cycle("root".into()))
        );
        let wrong = HashMap::from([("child".into(), excursion("other", vec![]))]);
        assert!(matches!(
            root.resolve_includes(&wrong),
            Err(IncludeError::IdMismatch { .. })
        ));
        let mut child = excursion("child", vec![]);
        child.version = 2;
        assert!(matches!(
            root.resolve_includes(&HashMap::from([("child".into(), child)])),
            Err(IncludeError::VersionMismatch { .. })
        ));
    }

    #[test]
    fn parses_include_and_rejects_combined_actions() {
        let yaml = "version: 1\nid: root\ntitle: Root\nsteps:\n  - include: child\n";
        let root: Excursion = serde_yaml::from_str(yaml).expect("valid include");
        assert_eq!(root.steps, vec![include("child")]);
        assert!(
            serde_yaml::from_str::<Excursion>(
                &yaml.replace("include: child", "include: child\n    say: hello")
            )
            .is_err()
        );
    }
}
