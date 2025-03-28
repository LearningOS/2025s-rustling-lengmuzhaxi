#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 检查 i16 是否在 0..=255 范围内，并转换成 u8
fn check_range(value: i16) -> Result<u8, IntoColorError> {
    if (0..=255).contains(&value) {
        Ok(value as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// Tuple 实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Ok(Color {
            red: check_range(tuple.0)?,
            green: check_range(tuple.1)?,
            blue: check_range(tuple.2)?,
        })
    }
}

// Array 实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Ok(Color {
            red: check_range(arr[0])?,
            green: check_range(arr[1])?,
            blue: check_range(arr[2])?,
        })
    }
}

// Slice 实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Ok(Color {
            red: check_range(slice[0])?,
            green: check_range(slice[1])?,
            blue: check_range(slice[2])?,
        })
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}
