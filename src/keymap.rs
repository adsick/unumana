#[derive(Debug, PartialEq)]
pub enum Keymap {
    Dvorak,
    Russian,
}

impl Keymap {
    pub fn convert(&self, sc: u32) -> char {
        match self {
            Keymap::Dvorak => sc.dvorak(),
            Keymap::Russian => sc.russian(),
        }
    }
    pub fn switch(&mut self) {
        *self = match *self {
            Keymap::Dvorak => Keymap::Russian,
            Keymap::Russian => Keymap::Dvorak,
        };
    }
    pub fn dvorak(sc: u32) -> char {
        match sc {
            2 => '1', //Num row
            3 => '2',
            4 => '3',
            5 => '4',
            6 => '5',
            7 => '6',
            8 => '7',
            9 => '8',
            10 => '9',
            11 => '0',
            12 => '[',
            13 => ']',

            15 => '\t', //TAB

            16 => '\'', //Q row
            17 => ',',
            18 => '.',
            19 => 'p',
            20 => 'y',
            21 => 'f',
            22 => 'g',
            23 => 'c',
            24 => 'r',
            25 => 'l',
            26 => '/',
            27 => '=',
            28 => '\n', //Enter

            30 => 'a', //A row
            31 => 'o',
            32 => 'e',
            33 => 'u',
            34 => 'i',
            35 => 'd',
            36 => 'h',
            37 => 't',
            38 => 'n',
            39 => 's',
            40 => '-',
            41 => '`',
            //42 => LShift
            43 => '\\', //Z row
            44 => ';',
            45 => 'q',
            46 => 'j',
            47 => 'k',
            48 => 'x',
            49 => 'b',
            50 => 'm',
            51 => 'w',
            52 => 'v',
            53 => 'z',

            //56 => LAlt
            57 => ' ', //SPACE
            //58 => Caps
            _ => Default::default(),
        }
    }
    pub fn russian(sc: u32) -> char {
        match sc {
            2 => '1', //Num row
            3 => '2',
            4 => '3',
            5 => '4',
            6 => '5',
            7 => '6',
            8 => '7',
            9 => '8',
            10 => '9',
            11 => '0',
            12 => '[',
            13 => ']',

            15 => '\t', //TAB

            16 => '??', //Q row
            17 => '??',
            18 => '??',
            19 => '??',
            20 => '??',
            21 => '??',
            22 => '??',
            23 => '??',
            24 => '??',
            25 => '??',
            26 => '??',
            27 => '??',
            28 => '\n', //Enter

            30 => '??', //A row
            31 => '??',
            32 => '??',
            33 => '??',
            34 => '??',
            35 => '??',
            36 => '??',
            37 => '??',
            38 => '??',
            39 => '??',
            40 => '??',
            41 => '`',
            //42 => LShift
            43 => '\\', //Z row
            44 => '??',
            45 => '??',
            46 => '??',
            47 => '??',
            48 => '??',
            49 => '??',
            50 => '??',
            51 => '??',
            52 => '??',
            53 => '.',

            //56 => LAlt
            57 => ' ', //SPACE
            //58 => Caps
            _ => Default::default(),
        }
    }
}

//LShift is 42
//LAlt is 56
//Caps is 58
//Backspace is 14

//57416 is Up
//57419 is Left
//57421 is Right
//57424 is Down

//this is for convenience
pub trait Convert {
    fn dvorak(self) -> char;
    fn russian(self) -> char;
}

impl Convert for u32 {
    fn dvorak(self) -> char {
        Keymap::dvorak(self)
    }

    fn russian(self) -> char {
        Keymap::russian(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lazy() {
        let mut c = 0;
        for i in 0..100 {
            let ch = Keymap::russian(i);
            if ch != '\x00' {
                print!("({}, {:?}), ", i, ch);
                c += 1;
            }
        }
        println!("total: {}", c);
    }
}
