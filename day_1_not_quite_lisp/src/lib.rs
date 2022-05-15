use std::{
    fs::File,
    io::{Read, Result},
};

#[allow(unused)]
fn load_file_content() -> Result<String> {
    let mut file = File::open("./input").expect("bad path");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

#[allow(unused)]
fn check_current_floor() -> Option<(i32)> {
    let m: String = load_file_content().unwrap();
    let mut floor: i32 = 0;
    let byte_val = m.into_bytes(); //.iter().map(|a| a as i32).collect();
    for i in byte_val {
        if i as i32 == 40 {
            floor += 1;
        } else if i as i32 == 41 {
            floor -= 1;
        }
    }
    Some(floor)
}

fn check_position_at_rasement() -> Option<usize> {
    let m: String = load_file_content().unwrap();
    let mut floor = 0;
    let mut pos = 0;
    let char_len = m.into_bytes().len();
    let byte_val = m.into_bytes();
    for i in 0..char_len {
        if byte_val[i] == 40 {
            floor += 1;
        } else if byte_val[i] == 41 {
            floor -= 1;
        }
        if floor == -1 {
            pos = i;
        }
    }
    Some(pos)
}

#[test]
fn test_check_current_floor() {
    assert_eq!(check_current_floor().unwrap(), 232)
}
