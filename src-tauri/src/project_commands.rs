use crate::models;

#[tauri::command]
pub async fn get_recent_projects() -> Vec<models::Project> {
    vec![
        models::Project {
            name: "InspectAnims".into(),
            path: "C:/Users/alex/mods/CSINSPECTMOD".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 0,
            },
        },
        models::Project {
            name: "Optium".into(),
            path: "C:/Users/alex/mods/optimization/optium".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 0,
            },
        },
        models::Project {
            name: "WarpathMod".into(),
            path: "C:/Users/alex/mods/servers/majorFFA".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 1,
            },
        },
    ]
}
