// Використовуйте осмислені назви змінних, функцій та структур
// Поганий приклад
fn F(x: i32) -> i32 { x * 2 }

// Гарний приклад
fn multiply_by_two(number: i32) -> i32 {
    number * 2
}

// Уникайте використання unwrap() без обробки помилок
fn read_config() {
    // Поганий приклад
    let text = std::fs::read_to_string("config.txt").unwrap();

    // Гарний приклад
    let text = std::fs::read_to_string("config.txt")
        .unwrap_or_else(|_| "Файл не знайдено".to_string());
    println!("Результат: {}", text);
}

// Використовуйте Option та Result для безпечної обробки даних
// Поганий приклад
fn divide_bad(a: f64, b: f64) -> f64 {
    a / b // можливий поділ на нуль
}

// Гарний приклад
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// Документуйте код за допомогою коментарів `///`
/// Обчислює факторіал числа.
/// Повертає None, якщо число є від’ємним.
fn factorial(n: i32) -> Option<u64> {
    if n < 0 { None } else { Some((1..=n as u64).product()) }
}

// Використовуйте cargo fmt для автоматичного форматування коду
// Поганий приклад
fn   sum (a:i32,b:i32)->i32{a+b}

// Гарний приклад
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// Розділяйте код на модулі
mod math {
    pub fn square(x: i32) -> i32 { x * x }
}

// Пишіть модульні тести
fn multiply(a: i32, b: i32) -> i32 { a * b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
}

// Уникайте дублювання коду (принцип DRY)
// Поганий приклад
fn area_examples_bad() {
    let area1 = 5.0 * 10.0;
    let area2 = 3.0 * 6.0;
    println!("{} {}", area1, area2);
}

// Гарний приклад
fn calculate_area(w: f64, h: f64) -> f64 { w * h }

fn area_examples_good() {
    let area1 = calculate_area(5.0, 10.0);
    let area2 = calculate_area(3.0, 6.0);
    println!("{} {}", area1, area2);
}

// Використовуйте borrow checker для безпечної роботи з пам’яттю
fn print_name(name: &String) {
    println!("Hello, {}", name);
}

fn borrow_example() {
    let user = String::from("Denys");
    print_name(&user);
}

// Використовуйте cargo clippy для автоматичного аналізу коду
fn clippy_example(x: bool) {
    // Поганий приклад
    if x == true { println!("True"); }

    // Гарний приклад
    if x {
        println!("True");
    }
}
