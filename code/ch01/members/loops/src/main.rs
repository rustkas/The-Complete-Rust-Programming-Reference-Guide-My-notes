use std::thread;
use std::time::{Duration, Instant};

const SECONDS:u64 = 1;
fn main() {
    let mut x = 1024;
    let mut y = 0;
    // Создаем мутабельную переменную для отслеживания времени
    let start_time = Instant::now();

    // Создаем поток для отсчета времени
    let time_thread = thread::spawn(move || {
        // Поток спит 3 секунды
        thread::sleep(Duration::from_secs(SECONDS));
    });

    loop {
        if x < 0 {
            break;
        }
        x += 1;
        x -= 1;
        y += 1;
        // Проверяем, прошло ли уже 3 секунды
        if start_time.elapsed() >= Duration::from_secs(SECONDS) {
            // Если прошло, прерываем цикл
            break;
        }
    }

    // Ожидаем завершения потока времени
    time_thread.join().expect("Time thread panicked");

    println!("{y} more runs to go");
}
