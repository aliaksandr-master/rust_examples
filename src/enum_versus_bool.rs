use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Trend {
    Up,
    Down,
}

#[test]
fn test_enum_vs_bool() {
    let mut arr_of_bool = Vec::new();
    let mut arr_of_trends = Vec::new();
    let mut arr_of_int: Vec<u8> = Vec::new();
    for e in 0..1000_000_000_usize {
        arr_of_bool.push(e % 2 == 0);
        arr_of_trends.push(if e % 2 == 0 { Trend::Up } else { Trend::Down });
        arr_of_int.push(if e % 2 == 0 { 1 } else { 0 });
    }

    let now = Instant::now();
    let mut eq_bool_cont = 0_usize;
    for e in arr_of_bool {
        if e {
            eq_bool_cont += 1;
        }
    }
    println!(">>BOOL: {:?}", now.elapsed());

    let now = Instant::now();
    let mut eq_up_count = 0_usize;
    for e in arr_of_trends {
        if e == Trend::Up {
            eq_up_count += 1;
        }
    }
    println!(">>ENUM: {:?}", now.elapsed());

    let now = Instant::now();
    let mut eq_1_count = 0_usize;
    for e in arr_of_int {
        if e == 1 {
            eq_1_count += 1;
        }
    }
    println!(">>INT:  {:?}", now.elapsed());

    assert_eq!(eq_bool_cont, eq_up_count);
    assert_eq!(eq_bool_cont, eq_1_count);
}
