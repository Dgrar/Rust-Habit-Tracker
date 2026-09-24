use std::io::stdin;

use clap::Parser;

use habit_core::Habit;
use habit_storage::get_contents;

// Creamos un comando con debug y Parser (Importado de CLAP)
#[derive(Debug, Parser)]
//
#[command(version, about = "Rastreador de hábitos")]
pub struct HabitsTrackerArgs {
    #[command(subcommand)]
    pub command: Commands,
}

// Se generan los comandos y los argumentos de cada uno
#[derive(Debug, Parser)]
pub enum Commands {
    /// Comando para añadir un hábito
    Add { name: String },
    /// Comando para eliminar un hábito
    Remove { name: String },
    /// Comando para completar un hábito
    Complete { name: String },
    /// Comando para mostrar los hábitos
    List,
}

fn main() {
    let args: HabitsTrackerArgs = HabitsTrackerArgs::parse();

    match args.command {
        Commands::Add { name } => {
            println!("Añadiendo {}", name);
            add_habit(&name)
        }
        Commands::Remove { name } => habit_storage::remove_from_data(name),
        Commands::List => habit_storage::list(),
        Commands::Complete { name } => habit_storage::update(name, None),
    }
}

fn add_habit(habit_name: &str) {
    let mut habit = Habit::new(0, habit_name.to_string());

    let mut response: String = "".to_string();

    println!("¿Ya has completado el hábito?: Y/n");

    stdin()
        .read_line(&mut response)
        .expect("No se ha podido leer");

    println!("{response}");

    if response.trim().to_uppercase() == "Y" {
        println!("¡Muy bien!, lo marco como completado");
        habit.set_completed(None);
    }

    habit_storage::write_habit(habit);
}
