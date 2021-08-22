use bimap::BiMap;
//BiMap is chosen for conflict detection.

//I would wish a double linked list mechanism here (sounds like the best option for this purpose),
//but implementation may be hard,
//(and yep, it is not enough just throw Keymaps into written d-linked list container)
//as I want to call methods on the corresponding object directly -
//this is great both from design and performance perspective
struct Keymap {
    keymaps: Vec<ConcreteKeymap>,
    current: usize,
}

impl Keymap {
    pub fn next(&mut self) {
        let len = self.keymaps.len();
        if self.current + 1 < len {
            self.current += 1
        } else {
            self.current = 0;
        }
    }
    pub fn name(&self) -> &str {
        &self.keymaps[self.current].name
    }
    pub fn new() -> Self {
        Keymap {
            keymaps: vec![ConcreteKeymap::dvorak(), ConcreteKeymap::russian()],
            current: 0,
        }
    }
    //badass one, but we assume that every incoming sc is valid (which may be not always the case)
    pub fn convert(&self, sc: u32) -> char {
        *self.get_current().map.get_by_left(&sc).unwrap()
    }
    ///returns the current concrete keymap
    fn get_current(&self) -> &ConcreteKeymap {
        &self.keymaps[self.current]
    }

    fn get_current_mut(&mut self) -> &mut ConcreteKeymap {
        &mut self.keymaps[self.current]
    }

    pub fn remap(&mut self, sc: u32, ch: char) {
        match self.get_current_mut().map.insert(sc, ch) {
            bimap::Overwritten::Neither => {#[cfg(any(test, debug_assertions))]println!("brand new map ({}<>{:?})", sc, ch)}
            bimap::Overwritten::Left(l, r) => {
                #[cfg(any(test, debug_assertions))]println!("scancode {} already exists in the map, it was pointing to  {:?}, now it points to {:?}\n", l, r, ch)
            }

            bimap::Overwritten::Right(l, r) => {
                #[cfg(any(test, debug_assertions))]
                println!(
                    "char {} existed in the map, overwritten scancode is {}\n",
                    r, l
                )
            }
            bimap::Overwritten::Pair(_, _) => (),
            bimap::Overwritten::Both((l1, r1), (l2, r2)) => {
                self.get_current_mut().map.insert(l2, r1);
                #[cfg(any(test, debug_assertions))]println!("swap. {:?} was binded to {} and {:?} was binded to {}.\nnow it is ({}<>{:?}) and ({}<>{:?})\n", r1, l1, r2, l2, sc, ch, l2, r1)
            }
        }
    }
}

#[derive(Default)]
struct ConcreteKeymap {
    name: String,
    map: BiMap<u32, char>,
}

impl ConcreteKeymap {
    fn dvorak() -> Self {
        let mut map = BiMap::with_capacity(64);

        for (sc, ch) in DVORAK {
            map.insert(sc, ch);
        }

        Self {
            name: "dvorak".to_string(),
            map,
        }
    }
    fn russian() -> Self {
        let mut map = BiMap::with_capacity(64);

        for (sc, ch) in RUSSIAN {
            map.insert(sc, ch);
        }
        Self {
            name: "russian".to_string(),
            map,
        }
    }
}

#[cfg(test)]
pub mod test_keymap {
    use super::Keymap;

    #[test]
    fn basic() {
        let mut k = Keymap::new();
        assert_eq!(k.name(), "dvorak");
        assert_eq!(k.convert(30), 'a');

        k.next();
        
        assert_eq!(k.convert(30), 'ф');
        assert_eq!(k.name(), "russian");
        
        k.next();
        
        assert_eq!(k.convert(30), 'a');
        assert_eq!(k.name(), "dvorak");
        
        k.remap(30, 'A');
        assert_eq!(k.convert(30), 'A');
        
        let mut k = Keymap::new();

        //here we test swapping 'a' and 'o'

        //in normal dvorak 30 is 'a' and 31 is 'o'
        //now we are about to change it...
        k.remap(31, 'a');
        assert_eq!(k.convert(31), 'a');
        assert_eq!(k.convert(30), 'o');
        //assert_eq!(k.convert(100), ' '); //with current realisation this will panic on unwrap
    }
}

const DVORAK: [(u32, char); 50] = [
    (2, '1'),
    (3, '2'),
    (4, '3'),
    (5, '4'),
    (6, '5'),
    (7, '6'),
    (8, '7'),
    (9, '8'),
    (10, '9'),
    (11, '0'),
    (12, '['),
    (13, ']'),
    (15, '\t'),
    (16, '\''),
    (17, ','),
    (18, '.'),
    (19, 'p'),
    (20, 'y'),
    (21, 'f'),
    (22, 'g'),
    (23, 'c'),
    (24, 'r'),
    (25, 'l'),
    (26, '/'),
    (27, '='),
    (28, '\n'),
    (30, 'a'),
    (31, 'o'),
    (32, 'e'),
    (33, 'u'),
    (34, 'i'),
    (35, 'd'),
    (36, 'h'),
    (37, 't'),
    (38, 'n'),
    (39, 's'),
    (40, '-'),
    (41, '`'),
    (43, '\\'),
    (44, ';'),
    (45, 'q'),
    (46, 'j'),
    (47, 'k'),
    (48, 'x'),
    (49, 'b'),
    (50, 'm'),
    (51, 'w'),
    (52, 'v'),
    (53, 'z'),
    (57, ' '),
];

const RUSSIAN: [(u32, char); 50] = [
    (2, '1'),
    (3, '2'),
    (4, '3'),
    (5, '4'),
    (6, '5'),
    (7, '6'),
    (8, '7'),
    (9, '8'),
    (10, '9'),
    (11, '0'),
    (12, '['),
    (13, ']'),
    (15, '\t'),
    (16, 'й'),
    (17, 'ц'),
    (18, 'у'),
    (19, 'к'),
    (20, 'е'),
    (21, 'н'),
    (22, 'г'),
    (23, 'ш'),
    (24, 'щ'),
    (25, 'з'),
    (26, 'х'),
    (27, 'ъ'),
    (28, '\n'),
    (30, 'ф'),
    (31, 'ы'),
    (32, 'в'),
    (33, 'а'),
    (34, 'п'),
    (35, 'р'),
    (36, 'о'),
    (37, 'л'),
    (38, 'д'),
    (39, 'ж'),
    (40, 'э'),
    (41, '`'),
    (43, '\\'),
    (44, 'я'),
    (45, 'ч'),
    (46, 'с'),
    (47, 'м'),
    (48, 'и'),
    (49, 'т'),
    (50, 'ь'),
    (51, 'б'),
    (52, 'ю'),
    (53, '.'),
    (57, ' '),
];
