
fn main() {
    println!("Hello, world!");
    
    let mut cycle_count = 0;
    const CYCLE_MAX : i32 = 4; 
    let mut cycle_type = CyclerType::Focus;
    let mut sec_count = 0;
    // 1 sec loop timer
    let timer = std::time::Duration::from_secs(1);
    loop {
        sec_count += 1;
        match cycle_type {
            CyclerType::Focus => {
                if sec_count == 25 * 60 {
                    cycle_type = CyclerType::ShortBreak;
                    sec_count = 0;
                }
                println!("Focus");
            },
            CyclerType::ShortBreak => {
                if sec_count == 5 * 60 {
                    cycle_type = CyclerType::Focus;
                    sec_count = 0;
                    cycle_count += 1;
                }
                println!("Short Break");
            },
            CyclerType::LongBreak => {
                if sec_count == 15 * 60 {
                    cycle_type = CyclerType::Focus;
                    sec_count = 0;
                    cycle_count += 1;
                }
                println!("Long Break");
            },
        }

        if cycle_count == CYCLE_MAX {
            cycle_type = CyclerType::LongBreak;
            cycle_count = 0;
        }
        std::thread::sleep(timer);
    }
}


enum CyclerType {
    Focus,
    ShortBreak,
    LongBreak,
}
