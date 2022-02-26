use std::thread;

fn main() {
    // Birkaç thread'in ortaklaşa kullanmak istediği bir sayı belirleyelim.
    let mut popular_number = 23_u32;

    // JoinHandle nesneleri için bir vector. Main'i bekletmek için kullanırız.
    let mut handlers = Vec::new();

    // 10 tane thread açalım
    for _ in 0..10 {
        handlers.push(thread::spawn(move || {

            println!(
                "{:?} Parametre : {}",
                thread::current().id(),
                popular_number
            );

            // Her bir thread popüler olan sayımızı 1 birim artırsın
            popular_number += 1;

            println!(
                "\t{:?} Yeni Değer : {}",
                thread::current().id(),
                popular_number
            );
        }));
    }

    for h in handlers {
        let _ = h.join();
    }

    println!("Sayının değeri {}", popular_number);
}
