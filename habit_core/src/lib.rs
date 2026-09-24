use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_completes_fine() {
        let mut test_habit = Habit {
            id: 0,
            title: "Comer Pizza".to_string(),
            streak: 0,
            status: StatusOption::Pending,
        };

        // Capturas el tiempo una sola vez en el test
        let time = SystemTime::now();
        test_habit.set_completed(Some(time));

        // Ahora ambos lados de la comparación tendrán exactamente el mismo nanosegundo
        assert_eq!(StatusOption::Completed(time), test_habit.status);
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum StatusOption {
    Pending, // Convención: PascalCase
    Completed(SystemTime),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Habit {
    pub id: i32,
    pub title: String,
    pub streak: u32,
    pub status: StatusOption, // Debe ser pub para leerlo desde el módulo de pruebas
}

impl Habit {
    pub fn new(id: i32, title: String) -> Habit {
        Habit {
            id,
            title,
            streak: 0,
            status: StatusOption::Pending,
        }
    }

    pub fn set_completed(&mut self, time: Option<SystemTime>) {
        let now_time = time.unwrap_or_else(SystemTime::now);
        if self.status == StatusOption::Pending {
            self.streak += 1
        }
        self.status = StatusOption::Completed(now_time)
    }

    pub fn get_time(&self) -> Option<SystemTime> {
        match self.status {
            StatusOption::Completed(time) => Some(time),
            StatusOption::Pending => None,
        }
    }
    pub fn refresh(&mut self) {
        self.status = StatusOption::Pending;
        self.streak = 0;
    }
}
