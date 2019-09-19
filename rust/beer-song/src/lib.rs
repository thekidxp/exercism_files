pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        y => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", y, y-1),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = verse(start);

    for number in (end..start).rev() {
        song = format!("{}\n{}", song, verse(number));
    }

    song
}

fn terse_verse(n: i32) -> String {
    let what_we_do_with_it = |number: i32| {
        if let 0 = number {
            format!("Go to the store and buy some more,")
        } else {
            format!(
                "Take {} down and pass it around,",
                if let 1 = number { "it" } else { "one" }
            )
        }
    };
    let bottle_phrase =
        |number: i32| format!("bottle{} of beer", if let 1 = number { "" } else { "s" });

    let number_string = n.to_string();
    let number_left = n - 1;
    let left_string = number_left.to_string();
    format!(
        "{first_num} {bottle} {wall}, {num} {bottle}.\n{action} {left} {second_bottle} {wall}.\n",
        first_num = if let 0 = n {
            "No more"
        } else {
            &number_string
        },
        num = if let 0 = n {
            "no more"
        } else {
            &number_string
        },
        bottle = bottle_phrase(n),
        action = what_we_do_with_it(n),
        left = match number_left {
            y if y < 0 => "99",
            0 => "no more",
            _ => &left_string,
        },
        second_bottle = bottle_phrase(number_left),
        wall = "on the wall"
    )
}
