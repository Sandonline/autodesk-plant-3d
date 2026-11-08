//! Windows registry read helpers for Autodesk Plant 3D.

pub const APP_KEY: &str = "Software\\Autodesk\\Autodesk Plant 3D";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Autodesk Plant 3D";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Autodesk Plant 3D") },
        RegValue { name: "Version",     value: String::from("v2.6.1") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Autodesk Plant 3D") },
        RegValue { name: "Publisher",       value: String::from("Autodesk") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Autodesk Plant 3D\\uninstall.exe\"") },
    ]
}
