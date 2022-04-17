
#[derive(Debug, Copy, Clone)]
pub enum Cardinall {
    _1_1,
    _1_2,
    _1_3,
    _1_4,
    _2_1,
    _2_2,
    _2_3,
    _2_4,
    _3_1,
    _3_2,
    _3_3,
    _3_4,
    _4_1,
    _4_2,
    _4_3,
    _4_4,
    Unknown,
}
impl Cardinall {

    pub fn get_value(self,side:u16) -> u16 {
        match side {
            2 => self.get_value2(),
            3 => self.get_value3(),
            4 => self.get_value4(),
            _ => 0,
        }
    }
    pub fn from(self, side:u16,val: u16) -> Cardinall {
        match side {
            2 => self.from2(val),
            3 => self.from3(val),
            4 => self.from4(val),
            _ => Cardinall::Unknown,
        }
    }

    /*
     get coord of square ((line,column) ,(line,column))
   */
    pub fn get_coord(self,side:u16) -> ((u16, u16), (u16, u16)) {
        match side {
            2 => self.get_coord2(),
            3 => self.get_coord3(),
            4 => self.get_coord4(),
            _ => ((0, 0), (0, 0)),
        }
    }
    /*
     get other square of the same line/row
    */
    pub fn _get_other(self,side:u16) -> Vec<Cardinall> {
        match side {
            2 => self._get_other2(),
            3 => self._get_other3(),
            4 => self._get_other4(),
            _ => Vec::new(),
        }
    }

    /*
    get all squares
    */
    pub fn get_all(self,side:u16) -> Vec<Cardinall> {
        match side {
            2 => self.get_all2(),
            3 => self.get_all3(),
            4 => self.get_all4(),
            _ => Vec::new(),
        }
    }
    /*
     get lines of a square
    */
    pub fn get_lines(self,side:u16) -> Vec<u16> {
        match side {
            2 => self.get_lines2(),
            3 => self.get_lines3(),
            4 => self.get_lines4(),
            _ => Vec::new(),
        }
    }        
    /*
    get columns of a square
    */
    pub fn get_columns(self,side:u16) -> Vec<u16> {
        match side {
            2 => self.get_columns2(),
            3 => self.get_columns3(),
            4 => self.get_columns4(),
            _ => Vec::new(),
        }
    }        
    /*
     get cells of square
    */
    pub fn _get_cells(self,side:u16) -> Vec<u16> {
        match side {
            2 => self._get_cells2(),
            3 => self._get_cells3(),
            4 => self._get_cells4(),
            _ => Vec::new(),
        }    }
}





//private functions
impl Cardinall {
    fn get_value4(self) -> u16 {
        match self {
            Cardinall::_1_1 => 1,
            Cardinall::_1_2 => 2,
            Cardinall::_1_3 => 3,
            Cardinall::_1_4 => 4,
            Cardinall::_2_1 => 5,
            Cardinall::_2_2 => 6,
            Cardinall::_2_3 => 7,
            Cardinall::_2_4 => 8,
            Cardinall::_3_1 => 9,
            Cardinall::_3_2 => 10,
            Cardinall::_3_3 => 11,
            Cardinall::_3_4 => 12,
            Cardinall::_4_1 => 13,
            Cardinall::_4_2 => 14,
            Cardinall::_4_3 => 15,
            Cardinall::_4_4 => 16,
            Cardinall::Unknown => 0,
        }
    }
    fn get_value3(self) -> u16 {
        match self {
            Cardinall::_1_1 => 1,
            Cardinall::_1_2 => 2,
            Cardinall::_1_3 => 3,
            Cardinall::_2_1 => 4,
            Cardinall::_2_2 => 5,
            Cardinall::_2_3 => 6,
            Cardinall::_3_1 => 7,
            Cardinall::_3_2 => 8,
            Cardinall::_3_3 => 9,
            Cardinall::Unknown => 0,
            _ => 0,
        }
    }
    fn get_value2(self) -> u16 {
        match self {
            Cardinall::_1_1 => 1,
            Cardinall::_1_2 => 2,
            Cardinall::_2_1 => 3,
            Cardinall::_2_2 => 4,
            Cardinall::Unknown => 0,
            _ => 0,
        }
    }

    fn from4(self, val: u16) -> Cardinall {
        match val {
            1 => Cardinall::_1_1,
            2 => Cardinall::_1_2,
            3 => Cardinall::_1_3,
            4 => Cardinall::_1_4,
            5 => Cardinall::_2_1,
            6 => Cardinall::_2_2,
            7 => Cardinall::_2_3,
            8 => Cardinall::_2_4,
            9 => Cardinall::_3_1,
            10 => Cardinall::_3_2,
            11 => Cardinall::_3_3,
            12 => Cardinall::_3_4,
            13 => Cardinall::_4_1,
            14 => Cardinall::_4_2,
            15 => Cardinall::_4_3,
            16 => Cardinall::_4_4,
            _ => Cardinall::Unknown, //default
        }
    }
    fn from3(self, val: u16) -> Cardinall {
        match val {
            1 => Cardinall::_1_1,
            2 => Cardinall::_1_2,
            3 => Cardinall::_1_3,
            4 => Cardinall::_2_1,
            5 => Cardinall::_2_2,
            6 => Cardinall::_2_3,
            7 => Cardinall::_3_1,
            8 => Cardinall::_3_2,
            9 => Cardinall::_3_3,
            _ => Cardinall::Unknown, //default
        }
    }
    fn from2(self, val: u16) -> Cardinall {
        match val {
            1 => Cardinall::_1_1,
            2 => Cardinall::_1_2,
            3 => Cardinall::_2_1,
            4 => Cardinall::_2_2,
            _ => Cardinall::Unknown, //default
        }
    }

    fn get_coord4(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinall::_1_1 => ((1, 1), (4, 4)),
            Cardinall::_1_2 => ((1, 5), (4, 8)),
            Cardinall::_1_3 => ((1, 9), (4, 12)),
            Cardinall::_1_4 => ((1, 13), (4, 16)),
            Cardinall::_2_1 => ((5, 1), (9, 4)),
            Cardinall::_2_2 => ((5, 5), (9, 8)),
            Cardinall::_2_3 => ((5, 9), (8, 12)),
            Cardinall::_2_4 => ((5, 13), (9, 16)),
            Cardinall::_3_1 => ((10, 1), (13, 4)),
            Cardinall::_3_2 => ((10, 5), (13, 8)),
            Cardinall::_3_3 => ((10, 9), (13, 12)),
            Cardinall::_3_4 => ((10, 13), (13, 16)),
            Cardinall::_4_1 => ((14, 1), (17, 4)),
            Cardinall::_4_2 => ((14, 5), (17, 8)),
            Cardinall::_4_3 => ((14, 9), (17, 12)),
            Cardinall::_4_4 => ((14, 13), (17, 16)),
            Cardinall::Unknown => ((0, 0), (0, 0)),
        }
    }
    fn get_coord3(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinall::_1_1 => ((1, 1), (3, 3)),
            Cardinall::_1_2 => ((1, 4), (3, 6)),
            Cardinall::_1_3 => ((1, 7), (3, 9)),
            Cardinall::_2_1 => ((4, 1), (6, 3)),
            Cardinall::_2_2 => ((4, 4), (6, 6)),
            Cardinall::_2_3 => ((4, 7), (6, 9)),
            Cardinall::_3_1 => ((7, 1), (9, 3)),
            Cardinall::_3_2 => ((7, 7), (9, 6)),
            Cardinall::_3_3 => ((7, 7), (9, 9)),
            Cardinall::Unknown => ((0, 0), (0, 0)),
            _ => ((0, 0), (0, 0)),
        }
    }
    fn get_coord2(self) -> ((u16, u16), (u16, u16)) {
        match self {
            Cardinall::_1_1 => ((1, 1), (2, 2)),
            Cardinall::_1_2 => ((1, 3), (2, 4)),
            Cardinall::_2_1 => ((3, 1), (4, 2)),
            Cardinall::_2_2 => ((3, 3), (4, 4)),
            Cardinall::Unknown => ((0, 0), (0, 0)),
            _ => ((0, 0), (0, 0)),
        }
    }

    fn _get_other4(self) -> Vec<Cardinall> {
        match self {
            Cardinall::_1_1 => vec![
                Cardinall::_1_2,
                Cardinall::_1_3,
                Cardinall::_1_4,
                Cardinall::_2_1,
                Cardinall::_3_1,
                Cardinall::_4_1,
            ],
            Cardinall::_1_2 => vec![
                Cardinall::_1_1,
                Cardinall::_1_3,
                Cardinall::_1_4,
                Cardinall::_2_2,
                Cardinall::_3_2,
                Cardinall::_4_2,
            ],
            Cardinall::_1_3 => vec![
                Cardinall::_1_1,
                Cardinall::_1_2,
                Cardinall::_1_4,
                Cardinall::_2_3,
                Cardinall::_3_3,
                Cardinall::_4_3,
            ],
            Cardinall::_1_4 => vec![
                Cardinall::_1_1,
                Cardinall::_1_2,
                Cardinall::_1_3,
                Cardinall::_2_4,
                Cardinall::_3_4,
                Cardinall::_4_4,
            ],
            Cardinall::_2_1 => vec![
                Cardinall::_2_2,
                Cardinall::_2_3,
                Cardinall::_2_4,
                Cardinall::_1_1,
                Cardinall::_3_1,
                Cardinall::_4_1,
            ],
            Cardinall::_2_2 => vec![
                Cardinall::_2_1,
                Cardinall::_2_3,
                Cardinall::_2_4,
                Cardinall::_1_2,
                Cardinall::_3_2,
                Cardinall::_4_2,
            ],
            Cardinall::_2_3 => vec![
                Cardinall::_2_1,
                Cardinall::_2_2,
                Cardinall::_2_4,
                Cardinall::_1_3,
                Cardinall::_3_3,
                Cardinall::_4_3,
            ],
            Cardinall::_2_4 => vec![
                Cardinall::_2_1,
                Cardinall::_2_2,
                Cardinall::_2_3,
                Cardinall::_1_4,
                Cardinall::_3_4,
                Cardinall::_4_4,
            ],
            Cardinall::_3_1 => vec![
                Cardinall::_3_2,
                Cardinall::_3_3,
                Cardinall::_3_4,
                Cardinall::_1_1,
                Cardinall::_2_1,
                Cardinall::_4_1,
            ],
            Cardinall::_3_2 => vec![
                Cardinall::_3_1,
                Cardinall::_3_3,
                Cardinall::_3_4,
                Cardinall::_1_2,
                Cardinall::_2_2,
                Cardinall::_4_2,
            ],
            Cardinall::_3_3 => vec![
                Cardinall::_3_1,
                Cardinall::_3_2,
                Cardinall::_3_4,
                Cardinall::_1_3,
                Cardinall::_2_3,
                Cardinall::_4_3,
            ],
            Cardinall::_3_4 => vec![
                Cardinall::_3_1,
                Cardinall::_3_2,
                Cardinall::_3_3,
                Cardinall::_1_4,
                Cardinall::_2_4,
                Cardinall::_4_4,
            ],
            Cardinall::_4_1 => vec![
                Cardinall::_4_2,
                Cardinall::_4_3,
                Cardinall::_4_4,
                Cardinall::_1_1,
                Cardinall::_2_1,
                Cardinall::_3_1,
            ],
            Cardinall::_4_2 => vec![
                Cardinall::_4_1,
                Cardinall::_4_3,
                Cardinall::_4_4,
                Cardinall::_1_2,
                Cardinall::_2_2,
                Cardinall::_2_2,
            ],
            Cardinall::_4_3 => vec![
                Cardinall::_4_1,
                Cardinall::_4_2,
                Cardinall::_4_4,
                Cardinall::_1_3,
                Cardinall::_3_3,
                Cardinall::_3_3,
            ],
            Cardinall::_4_4 => vec![
                Cardinall::_4_1,
                Cardinall::_4_2,
                Cardinall::_4_3,
                Cardinall::_1_4,
                Cardinall::_2_4,
                Cardinall::_2_4,
            ],
            Cardinall::Unknown => Vec::new(),
        }
    }
    fn _get_other3(self) -> Vec<Cardinall> {
        match self {
            Cardinall::_1_1 => vec![
                Cardinall::_1_2,
                Cardinall::_1_3,
                Cardinall::_2_1,
                Cardinall::_3_1,
            ],
            Cardinall::_1_2 => vec![
                Cardinall::_1_1,
                Cardinall::_1_3,
                Cardinall::_2_2,
                Cardinall::_3_2,
            ],
            Cardinall::_1_3 => vec![
                Cardinall::_1_1,
                Cardinall::_1_2,
                Cardinall::_2_3,
                Cardinall::_3_3,
            ],
            Cardinall::_2_1 => vec![
                Cardinall::_1_1,
                Cardinall::_3_1,
                Cardinall::_2_2,
                Cardinall::_2_3,
            ],
            Cardinall::_2_2 => vec![
                Cardinall::_1_2,
                Cardinall::_3_2,
                Cardinall::_2_1,
                Cardinall::_2_3,
            ],
            Cardinall::_2_3 => vec![
                Cardinall::_1_3,
                Cardinall::_3_3,
                Cardinall::_2_2,
                Cardinall::_2_1,
            ],
            Cardinall::_3_1 => vec![
                Cardinall::_1_1,
                Cardinall::_2_1,
                Cardinall::_3_2,
                Cardinall::_3_3,
            ],
            Cardinall::_3_2 => vec![
                Cardinall::_3_1,
                Cardinall::_3_3,
                Cardinall::_2_2,
                Cardinall::_1_2,
            ],
            Cardinall::_3_3 => vec![
                Cardinall::_3_1,
                Cardinall::_3_2,
                Cardinall::_2_3,
                Cardinall::_1_3,
            ],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }
    fn _get_other2(self) -> Vec<Cardinall> {
        match self {
            Cardinall::_1_1 => vec![Cardinall::_1_2, Cardinall::_2_1],
            Cardinall::_1_2 => vec![Cardinall::_1_1, Cardinall::_2_2],
            Cardinall::_2_1 => vec![Cardinall::_1_1, Cardinall::_2_2],
            Cardinall::_2_2 => vec![Cardinall::_1_2, Cardinall::_2_1],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn get_all4(self) -> Vec<Cardinall> {
        vec![
            Cardinall::_1_1,
            Cardinall::_1_2,
            Cardinall::_1_3,
            Cardinall::_1_4,
            Cardinall::_2_1,
            Cardinall::_2_2,
            Cardinall::_2_3,
            Cardinall::_2_4,
            Cardinall::_3_1,
            Cardinall::_3_2,
            Cardinall::_3_3,
            Cardinall::_3_4,
            Cardinall::_4_1,
            Cardinall::_4_2,
            Cardinall::_4_3,
            Cardinall::_4_4,
        ]
    }
    fn get_all3(self) -> Vec<Cardinall> {
        vec![
            Cardinall::_1_1,
            Cardinall::_1_2,
            Cardinall::_1_3,
            Cardinall::_2_1,
            Cardinall::_2_2,
            Cardinall::_2_3,
            Cardinall::_3_1,
            Cardinall::_3_2,
            Cardinall::_3_3,
        ]
    }
    fn get_all2(self) -> Vec<Cardinall> {
        vec![
            Cardinall::_1_1,
            Cardinall::_1_2,
            Cardinall::_2_1,
            Cardinall::_2_2,
        ]
    }

    fn get_lines4(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2, 3, 4],
            Cardinall::_1_2 => vec![1, 2, 3, 4],
            Cardinall::_1_3 => vec![1, 2, 3, 4],
            Cardinall::_1_4 => vec![1, 2, 3, 4],
            Cardinall::_2_1 => vec![5, 6, 7, 8],
            Cardinall::_2_2 => vec![5, 6, 7, 8],
            Cardinall::_2_3 => vec![5, 6, 7, 8],
            Cardinall::_2_4 => vec![5, 6, 7, 8],
            Cardinall::_3_1 => vec![9, 10, 11, 12],
            Cardinall::_3_2 => vec![9, 10, 11, 12],
            Cardinall::_3_3 => vec![9, 10, 11, 12],
            Cardinall::_3_4 => vec![9, 10, 11, 12],
            Cardinall::_4_1 => vec![13, 14, 15, 16],
            Cardinall::_4_2 => vec![13, 14, 15, 16],
            Cardinall::_4_3 => vec![13, 14, 15, 16],
            Cardinall::_4_4 => vec![13, 14, 15, 16],
            Cardinall::Unknown => Vec::new(),
        }
    }
    fn get_lines3(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2, 3],
            Cardinall::_1_2 => vec![1, 2, 3],
            Cardinall::_1_3 => vec![1, 2, 3],
            Cardinall::_2_1 => vec![4, 5, 6],
            Cardinall::_2_2 => vec![4, 5, 6],
            Cardinall::_2_3 => vec![4, 5, 6],
            Cardinall::_3_1 => vec![7, 8, 9],
            Cardinall::_3_2 => vec![7, 8, 9],
            Cardinall::_3_3 => vec![7, 8, 9],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }
    fn get_lines2(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2],
            Cardinall::_1_2 => vec![1, 2],
            Cardinall::_2_1 => vec![3, 4],
            Cardinall::_2_2 => vec![3, 4],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn get_columns4(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2, 3, 4],
            Cardinall::_2_1 => vec![1, 2, 3, 4],
            Cardinall::_3_1 => vec![1, 2, 3, 4],
            Cardinall::_4_1 => vec![1, 2, 3, 4],
            Cardinall::_1_2 => vec![5, 6, 7, 8],
            Cardinall::_2_2 => vec![5, 6, 7, 8],
            Cardinall::_3_2 => vec![5, 6, 7, 8],
            Cardinall::_4_2 => vec![5, 6, 7, 8],
            Cardinall::_1_3 => vec![9, 10, 11, 12],
            Cardinall::_2_3 => vec![9, 10, 11, 12],
            Cardinall::_3_3 => vec![9, 10, 11, 12],
            Cardinall::_4_3 => vec![9, 10, 11, 12],
            Cardinall::_1_4 => vec![13, 14, 15, 16],
            Cardinall::_2_4 => vec![13, 14, 15, 16],
            Cardinall::_3_4 => vec![13, 14, 15, 16],
            Cardinall::_4_4 => vec![13, 14, 15, 16],
            Cardinall::Unknown => Vec::new(),
        }
    }
    fn get_columns3(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2, 3],
            Cardinall::_2_1 => vec![1, 2, 3],
            Cardinall::_3_1 => vec![1, 2, 3],
            Cardinall::_1_2 => vec![4, 5, 6],
            Cardinall::_2_2 => vec![4, 5, 6],
            Cardinall::_3_2 => vec![4, 5, 6],
            Cardinall::_2_3 => vec![7, 8, 9],
            Cardinall::_1_3 => vec![7, 8, 9],
            Cardinall::_3_3 => vec![7, 8, 9],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }
    fn get_columns2(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![1, 2],
            Cardinall::_2_1 => vec![1, 2],
            Cardinall::_1_2 => vec![3, 4],
            Cardinall::_2_2 => vec![3, 4],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn _get_cells4(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![0, 1, 2, 3, 16, 17, 18, 19, 32, 33, 34, 35],
            Cardinall::_1_2 => vec![4, 5, 6, 7, 20, 21, 22, 23, 36, 37, 38, 39],
            Cardinall::_1_3 => vec![8, 9, 10, 11, 24, 25, 26, 27, 40, 41, 42, 43],
            Cardinall::_1_4 => vec![12, 13, 14, 15, 28, 29, 30, 31, 44, 45, 46, 47],
            Cardinall::_2_1 => vec![49, 49, 50, 51, 64, 65, 66, 67, 80, 81, 82, 83],
            Cardinall::_2_2 => vec![52, 53, 54, 55, 68, 69, 70, 71, 84, 85, 86, 87],
            Cardinall::_2_3 => vec![56, 57, 58, 59, 72, 73, 74, 75, 88, 89, 90, 91],
            Cardinall::_2_4 => vec![60, 61, 62, 63, 76, 77, 78, 79, 92, 93, 94, 95],
            Cardinall::_3_1 => vec![96, 97, 98, 99, 112, 113, 114, 115, 128, 129, 130, 131],
            Cardinall::_3_2 => vec![100, 101, 102, 103, 116, 117, 118, 119, 132, 133, 134, 135],
            Cardinall::_3_3 => vec![104, 105, 106, 107, 120, 121, 122, 123, 136, 137, 138, 139],
            Cardinall::_3_4 => vec![108, 109, 110, 111, 124, 125, 126, 127, 140, 141, 142, 143],
            Cardinall::_4_1 => vec![144, 145, 146, 147, 160, 161, 162, 163, 176, 177, 178, 179],
            Cardinall::_4_2 => vec![148, 149, 150, 151, 164, 165, 166, 167, 180, 181, 182, 183],
            Cardinall::_4_3 => vec![152, 153, 154, 155, 168, 169, 170, 171, 184, 185, 186, 187],
            Cardinall::_4_4 => vec![156, 157, 158, 159, 172, 173, 174, 175, 188, 189, 191, 495],
            Cardinall::Unknown => Vec::new(),
        }
    }
   fn _get_cells3(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
            Cardinall::_1_2 => vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
            Cardinall::_1_3 => vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
            Cardinall::_2_1 => vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            Cardinall::_2_2 => vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            Cardinall::_2_3 => vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            Cardinall::_3_1 => vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            Cardinall::_3_2 => vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            Cardinall::_3_3 => vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }
    fn _get_cells2(self) -> Vec<u16> {
        match self {
            Cardinall::_1_1 => vec![0, 1, 4, 5],
            Cardinall::_1_2 => vec![2, 3, 6, 7],
            Cardinall::_2_1 => vec![8, 9, 12, 13],
            Cardinall::_2_2 => vec![10, 11, 14, 15],
            Cardinall::Unknown => Vec::new(),
            _ => Vec::new(),
        }
    }
}