use chrono::{DateTime, Local};
use habit_core::{Habit, StatusOption};
use std::{
    fs,
    path::Path,
    time::{Duration, SystemTime},
};

static PATH: &str = r"C:\Users\Roberto\Documents\Código\RUST\Proyectos_begginer\Habits_tracker_CLI\habit_storage\data\data.json";

pub fn write_habit(mut habit: Habit) {
    let mut previous = get_contents();

    habit.id = previous.len() as i32;

    if !previous
        .iter()
        .any(|previous_habit| previous_habit.title == habit.title)
    {
        previous.push(habit);
    }

    let json = serde_json::to_string_pretty(&previous).expect("No se ha podido parsear");

    if let Some(parent) = Path::new(PATH).parent() {
        let _ = fs::create_dir_all(parent);
    }

    fs::write(PATH, json).expect("No se ha podido escribir");
    println!("Written!")
}

pub fn get_contents() -> Vec<Habit> {
    let lines = match fs::read_to_string(PATH) {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };

    if lines.trim().is_empty() {
        return Vec::new();
    }

    let habits: Vec<Habit> = match serde_json::from_str(&lines) {
        Ok(habits) => habits,
        Err(_) => return Vec::new(),
    };

    habits
}

pub fn remove_from_data(name: String) {
    let mut content = get_contents();
    content.retain(|habit| habit.title != name);
    let json = serde_json::to_string_pretty(&content).expect("No se ha podido parsear");
    fs::write(PATH, json).expect("No se pudo escribir");
}

// NUEVA FUNCIÓN AUXILIAR: Sincroniza el estado del archivo si el tiempo expiró
fn auto_refresh_habits(content: &mut Vec<Habit>) -> bool {
    let mut hubo_cambios = false;

    for habit in content.iter_mut() {
        if let StatusOption::Completed(s_time) = habit.status {
            if SystemTime::now().duration_since(s_time).unwrap_or_default()
                > Duration::from_secs(60 * 60 * 24)
            {
                habit.refresh(); // Supongo que esto reinicia racha o cambia a Pending
                hubo_cambios = true;
            }
        }
    }

    if hubo_cambios {
        let json = serde_json::to_string_pretty(content).expect("No se ha podido parsear");
        let _ = fs::write(PATH, json);
    }

    hubo_cambios
}

pub fn list() {
    let mut content = get_contents();

    if content.is_empty() {
        println!("\n📭 ¡No hay hábitos registrados aún! Añade uno para empezar.");
        return;
    }

    // Ejecutamos el refresco automático antes de pintar en pantalla
    auto_refresh_habits(&mut content);

    println!("\n================📋 LISTA DE HÁBITOS ================");
    println!("{:<20} | {:<8} | {}", "TÍTULO", "RACHA", "ESTADO");
    println!("----------------------------------------------------");

    for habit in content {
        match habit.status {
            StatusOption::Pending => {
                println!("{:<20} | {:<8} | ⏳ Pendiente", habit.title, habit.streak)
            }
            StatusOption::Completed(s_time) => {
                let real_time: DateTime<Local> = s_time.into();
                let hour = real_time.format("%H:%M:%S").to_string();
                let date = real_time.format("%d/%m/%Y").to_string();

                println!(
                    "{:<20} | {:<8} | ✅ Completado ({} - {})",
                    habit.title, habit.streak, date, hour
                );
            }
        }
    }
    println!("====================================================\n");
}

pub fn update_habit_direct(title: String, new_habit: Habit) {
    let mut content = get_contents();

    if let Some(habit) = content.iter_mut().find(|h| h.title == title) {
        *habit = new_habit; // Reemplaza los valores del hábito encontrado de forma segura

        let json = serde_json::to_string_pretty(&content).expect("No se ha podido parsear");
        fs::write(PATH, json).expect("No se pudo escribir");
        println!("¡Hábito '{}' actualizado con éxito!", title);
    } else {
        println!("Error, hábito no encontrado");
    }
}

fn flush() {
    let _ = fs::write(PATH, "[]");
}

// ... Tus pruebas unitarias se mantienen intactas aquí abajo ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes() {
        flush();
        let test_habit = Habit::new(0, "Almorzar".to_string());

        write_habit(test_habit);

        assert_eq!(get_contents()[0], Habit::new(0, "Almorzar".to_string()))
    }

    #[test]
    fn not_double_writes() {
        flush();
        let test_habit1 = Habit::new(0, "Almorzar".to_string());

        write_habit(test_habit1);

        let test_habit2 = Habit::new(0, "Almorzar".to_string());

        write_habit(test_habit2);
        assert_eq!(get_contents().len(), 1)
    }

    #[test]
    fn it_removes() {
        flush();
        let test_habit = Habit::new(0, "Almorzar".to_string());

        write_habit(test_habit);

        remove_from_data("Almorzar".to_string());

        assert_eq!(get_contents().len(), 0)
    }

    #[test]
    fn writes_two_things() {
        flush();
        let test_habit1 = Habit::new(43, "Almorzar".to_string());

        write_habit(test_habit1);

        let test_habit2 = Habit::new(1, "Comer".to_string());

        write_habit(test_habit2);

        assert_eq!(get_contents().len(), 2)
    }
}

pub fn update(title: String, mut new_habit: Option<Habit>) {
    let mut content = get_contents();

    if let Some(mut habit) = content.iter_mut().find(|h| h.title == title) {
        match new_habit {
            Some(mut hb) => habit = &mut hb,
            None => habit.set_completed(None),
        }

        let json = serde_json::to_string_pretty(&content).expect("No se ha podido parsear");
        fs::write(PATH, json).expect("No se pudo escribir");

        println!("¡Hábito '{}' actualizado con éxito!", title);
    } else {
        println!("Error, hábito no encontrado");
    }
}
