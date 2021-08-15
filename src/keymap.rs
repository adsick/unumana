#[derive(Debug, PartialEq)]
pub enum Keymap {
    Dvorak,
    Russian,
}

impl Keymap {
    pub fn convert(&self, sc: u32) -> char {
        match self {
            Keymap::Dvorak => Keymap::dvorak(sc),
            Keymap::Russian => Keymap::russian(sc),
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

            16 => 'й', //Q row
            17 => 'ц',
            18 => 'у',
            19 => 'к',
            20 => 'е',
            21 => 'н',
            22 => 'г',
            23 => 'ш',
            24 => 'щ',
            25 => 'з',
            26 => 'х',
            27 => 'ъ',
            28 => '\n', //Enter

            30 => 'ф', //A row
            31 => 'ы',
            32 => 'в',
            33 => 'а',
            34 => 'п',
            35 => 'р',
            36 => 'о',
            37 => 'л',
            38 => 'д',
            39 => 'ж',
            40 => 'э',
            41 => '`',
            //42 => LShift
            43 => '\\', //Z row
            44 => 'я',
            45 => 'ч',
            46 => 'с',
            47 => 'м',
            48 => 'и',
            49 => 'т',
            50 => 'ь',
            51 => 'б',
            52 => 'ю',
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
