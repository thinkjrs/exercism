struct RangeIterator {
    current: u32,
    end: u32,
}

impl RangeIterator {
    fn new(start: u32, end: u32) -> Self {
        RangeIterator {
            current: start,
            end,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end - 1 {
            let result = Some(self.current);
            self.current -= 1;
            result
        } else {
            None
        }
    }
}

pub fn verse(n: u32) -> String {
    //todo!("emit verse {n}")
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.");
    }
    if n == 1 {
        return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    if n < 3 {
        return format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
}

pub fn sing(start: u32, end: u32) -> String {
    //todo!("sing verses {start} to {end}, inclusive")
    let mut lyrics = String::new();
    for n in (end..=start).rev() {
        lyrics.push_str(&verse(n));
        if n != end {
            lyrics.push_str("\n");
        }
    }
    lyrics
}
