# Diseño del proyecto:
## 3 partes:
- **CLI**: La interfaz principal y lo que se encarga de recoger los argumentos
- **CORE**: Se encarga de la lógica principal (Añadir y eliminar rutinas, completar, rachas...)
- **STORAGE**: Guarda los datos en un JSON

## Estructura de hábito:

```rust
enum Status{
    pending,
    completed(SystemTime)
}

struct Habit{
    id: i32
    title: String
    streak: u32
    status: StatusOption
}
```
