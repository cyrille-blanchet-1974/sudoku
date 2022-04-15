#[derive(Debug, Copy, Clone)]
pub struct Metrics {
    squareside: u8,
    nbcolumn: u8,
    nbline: u8,
    gridsize: u16,
    max: u8,
}

impl Metrics {
    pub fn new(side: u8) -> Metrics {
        let columnsize = side * side;
        let linesize = side * side;
        let gridsize = (columnsize * linesize) as u16;
        let max = side * side;
        Metrics {
            squareside: side,
            nbcolumn: columnsize,
            nbline: linesize,
            gridsize,
            max,
        }
    }
    pub fn get_square_side(&self) -> u8 {
        self.squareside
    }
    pub fn get_nb_column(&self) -> u8 {
        self.nbcolumn
    }
    pub fn get_nb_line(&self) -> u8 {
        self.nbline
    }
    pub fn get_grid_size(&self) -> u16 {
        self.gridsize
    }
    pub fn get_max(&self) -> u8 {
        self.max
    }
}

#[test]
fn check() {
    let m = Metrics::new(3);
    assert_eq!(3, m.get_square_side());
    assert_eq!(9, m.get_nb_column());
    assert_eq!(9, m.get_nb_line());
    assert_eq!(81, m.get_grid_size());
    assert_eq!(9, m.get_max());

    let m = Metrics::new(2);
    assert_eq!(2, m.get_square_side());
    assert_eq!(4, m.get_nb_column());
    assert_eq!(4, m.get_nb_line());
    assert_eq!(16, m.get_grid_size());
    assert_eq!(4, m.get_max());

    let m = Metrics::new(4);
    assert_eq!(4, m.get_square_side());
    assert_eq!(16, m.get_nb_column());
    assert_eq!(16, m.get_nb_line());
    assert_eq!(256, m.get_grid_size());
    assert_eq!(16, m.get_max());
}
