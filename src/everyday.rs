pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub fn temp_converter(temp: f64, from_unit: TempUnit, to_unit: TempUnit) -> f64 {
    match from_unit {
        TempUnit::Celsius => match to_unit {
            TempUnit::Celsius => temp,
            TempUnit::Fahrenheit => (temp * 9.0 / 5.0) + 32.0,
            TempUnit::Kelvin => temp - 273.0,
        },
        TempUnit::Fahrenheit => match to_unit {
            TempUnit::Celsius => (5.0 / 9.0) * (temp - 32.0),
            TempUnit::Fahrenheit => temp,
            TempUnit::Kelvin => temp_converter(
                (5.0 / 9.0) * (temp - 32.0),
                TempUnit::Celsius,
                TempUnit::Kelvin,
            ),
        },
        TempUnit::Kelvin => match to_unit {
            TempUnit::Celsius => temp + 273.0,
            TempUnit::Fahrenheit => {
                temp_converter(temp + 273.0, TempUnit::Celsius, TempUnit::Fahrenheit)
            }
            TempUnit::Kelvin => temp,
        },
    }
}


pub fn largest_challenge<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    let mut i = 0;

    loop {
        if let Some(j) = list.get(i) {
            i += 1;
            if j > largest {
                largest = j;
            }
        } else {
            break
        }
    }

    largest
       
}

pub trait Len {
    fn len(&self) -> usize;
}

pub fn longer<'a>(item1: &'a impl Len, item2: &'a impl Len) -> &'a impl Len {
    if item1.len() >= item2.len() {
        item1
    } else {
        item1
    }
}

pub fn next_some<T>(iter: Box<dyn Iterator<Item = Option<T>>>, start: usize) -> Option<T> {
    let temp = iter.skip(start);
    for next in temp {
        if let Some(ret) = next {
            return Some(ret)
        }
    }

    // Made it through, no Some. 
    None
}
