use super::accessor::*;
use super::cell::*;
use super::constant::*;
use super::grid::*;
use std::convert::TryInto;

pub struct ResolverLvl4 {
    acc: Accessor, //methods to retreive cells by coordinates
    debug : bool,
}

impl ResolverLvl4 {
    pub fn new(debug : bool) -> ResolverLvl4 {
        ResolverLvl4 {
            acc: Accessor::new(),
            debug ,
        }
    }

    /*
    X-wing resolve
    return true if found one or more
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        print!("Lvl4->");
        let mut solve_one_at_least = false;
        if self.resolve_line(g){
            solve_one_at_least=true;
            //while not finish
            solve_one_at_least=false;
        }
        if self.resolve_column(g){
            solve_one_at_least=true;
            //while not finish
            solve_one_at_least=false;
        }
        println!();
        
        solve_one_at_least
    }

    fn resolve_line(&mut self, g: &mut Grid) -> bool {
        //loop values
        false
    }
    fn resolve_column(&mut self, g: &mut Grid) -> bool {
        let mut trouve=false;
        //loop values
        for v in 1..=MAX {
            if self.debug{
                println!("try value {} ",v);
            }
            let val: usize = v.try_into().unwrap();            
            let mut tab = Vec::new();
            //loop columns
            'cols: for column in 1..=COLUMNSIZE {
                let mut t = (0,0);// count   and positions
                //count is nb of cell with val in possibles
                //positions is 'binairy' positions of line with val possible 
                let mut p : u32=100_000_000;
                for line in 1..=LINESIZE {
                    let pos = coord_to_pos(line,column);
                    let cell: &mut Cell = &mut (g.get_cell(pos));
                    if cell.is_resolved() {
                        match cell.get_answer() {
                            None => {}
                            Some(x) => {
                                if x == v {
                                    //println!("val {} already resolved in {} {}",val,line,column);
                                    tab.push((0,0));
                                    //println!("val {} col {} -> {:?}",val,column,(0,0));
                                    //ignore this column
                                    continue 'cols;
                                }
                            }
                        };        
                    }
                    if cell.is_a_possible(val) {
                        t.0 = t.0+1;
                        t.1 = t.1+ p;
                    }
                    //line 1  => 1 000 000
                    //line 2  =>   100 000
                    //...
                    //line 8  =>        10
                    //line 9  =>         1
                    p=p/10;
                }
                tab.push(t);
                //println!("val {} col {} -> {:?}",val,column,t);                
                //check lines with value resolved if only 2 lines keep
                //if 2 columns (and only two) with same two lines remove val off possible for other lines of the two cols
            }
            //println!("part 2");
            //check if we find 2 cols with only 2 count and the same position
            //loop on tab
            for column in 1..=COLUMNSIZE {
                let c: usize = column.try_into().unwrap();
                let t = tab[c-1];
                //find a cell with count = 2
                if t.0 == 2{                    
                    if self.debug{
                        print!("count 2 -> col {} -> {:?}",column,t);
                    }
                    //then search another one
                    for column2 in column+1..=COLUMNSIZE {
                        let c2: usize = column2.try_into().unwrap();
                        let t2 = tab[c2-1];
                        if self.debug{
                            print!("col2 {} -> {:?}",column2,t2);
                        }
                        //find a cell with count = 2
                        if t2.0 == 2 && t.1==t2.1 {                            
                            //found 2 cols each of them has value v possible in the sames lines
                            //So we can remove value v of all other cells of this two lines
                            println!("xwing found for val {} in columns {}({:?}) and {}({:?}) ",val,c,t,c2,t2);
                            trouve=true;
                        }
                    }
                    if self.debug{
                        println!("");
                    }
                }
            }
        }
        trouve
    }

}


#[test]
fn test() {
    let mut g = Grid::default();
    g.set_val(1, 1, 6, CellType::ORIGIN);
    g.set_val(1, 2, 7, CellType::ORIGIN);
    g.set_val(1, 3, 9, CellType::ORIGIN);
    g.set_val(1, 4, 4, CellType::ORIGIN);
    g.set_val(1, 5, 1, CellType::ORIGIN);
    g.set_val(1, 6, 8, CellType::ORIGIN);
    g.set_val(1, 7, 3, CellType::ORIGIN);
    g.set_val(1, 8, 5, CellType::ORIGIN);
    g.set_val(1, 9, 2, CellType::ORIGIN);
    g.set_val(2, 1, 2, CellType::ORIGIN);
    g.set_val(2, 2, 4, CellType::ORIGIN);
    g.set_val(2, 3, 4, CellType::ORIGIN);
    g.set_val(2, 4, 3, CellType::ORIGIN);
    g.set_val(2, 5, 9, CellType::ORIGIN);
    g.set_val(2, 6, 5, CellType::ORIGIN);
    g.set_val(2, 7, 7, CellType::ORIGIN);
    g.set_val(2, 8, 6, CellType::ORIGIN);
    g.set_val(2, 9, 1, CellType::ORIGIN);
    g.set_val(3, 1, 1, CellType::ORIGIN);
    g.set_val(3, 2, 5, CellType::ORIGIN);
    g.set_val(3, 3, 3, CellType::ORIGIN);
    g.set_val(3, 4, 7, CellType::ORIGIN);
    g.set_val(3, 5, 6, CellType::ORIGIN);
    g.set_val(3, 6, 2, CellType::ORIGIN);
    g.set_val(3, 7, 9, CellType::ORIGIN);
    g.set_val(3, 8, 8, CellType::ORIGIN);
    g.set_val(3, 9, 4, CellType::ORIGIN);
    //ori.set_val(4, 1, 1, CellType::ORIGIN);
    //ori.set_val(4, 2, 1, CellType::ORIGIN);
    //ori.set_val(4, 3, 1, CellType::ORIGIN);
    //ori.set_val(4, 4, 1, CellType::ORIGIN);
    //ori.set_val(4, 5, 1, CellType::ORIGIN);
    //ori.set_val(4, 6, 1, CellType::ORIGIN);
    g.set_val(4, 7, 4, CellType::ORIGIN);
    g.set_val(4, 8, 2, CellType::ORIGIN);
    //ori.set_val(4, 9, 1, CellType::ORIGIN);
    g.set_val(5, 1, 7, CellType::ORIGIN);
    //ori.set_val(5, 2, 1, CellType::ORIGIN);
    g.set_val(5, 3, 4, CellType::ORIGIN);
    //ori.set_val(5, 4, 1, CellType::ORIGIN);
    g.set_val(5, 5, 8, CellType::ORIGIN);
    g.set_val(5, 6, 1, CellType::ORIGIN);
    g.set_val(5, 7, 5, CellType::ORIGIN);
    g.set_val(5, 8, 3, CellType::ORIGIN);
    g.set_val(5, 9, 6, CellType::ORIGIN);
    //ori.set_val(6, 1, 1, CellType::ORIGIN);
    //ori.set_val(6, 2, 1, CellType::ORIGIN);
    //ori.set_val(6, 3, 1, CellType::ORIGIN);
    //ori.set_val(6, 4, 1, CellType::ORIGIN);
    //ori.set_val(6, 5, 1, CellType::ORIGIN);
    //ori.set_val(6, 6, 1, CellType::ORIGIN);
    g.set_val(6, 7, 1, CellType::ORIGIN);
    g.set_val(6, 8, 7, CellType::ORIGIN);
    //ori.set_val(6, 9, 1, CellType::ORIGIN);
    g.set_val(7, 1, 5, CellType::ORIGIN);
    g.set_val(7, 2, 8, CellType::ORIGIN);
    g.set_val(7, 3, 7, CellType::ORIGIN);
    g.set_val(7, 4, 1, CellType::ORIGIN);
    g.set_val(7, 5, 2, CellType::ORIGIN);
    g.set_val(7, 6, 9, CellType::ORIGIN);
    g.set_val(7, 7, 6, CellType::ORIGIN);
    g.set_val(7, 8, 4, CellType::ORIGIN);
    g.set_val(7, 9, 3, CellType::ORIGIN);
    g.set_val(8, 1, 4, CellType::ORIGIN);
    g.set_val(8, 2, 6, CellType::ORIGIN);
    g.set_val(8, 3, 1, CellType::ORIGIN);
    g.set_val(8, 4, 8, CellType::ORIGIN);
    g.set_val(8, 5, 3, CellType::ORIGIN);
    g.set_val(8, 6, 7, CellType::ORIGIN);
    g.set_val(8, 7, 2, CellType::ORIGIN);
    g.set_val(8, 8, 9, CellType::ORIGIN);
    g.set_val(8, 9, 5, CellType::ORIGIN);
    g.set_val(9, 1, 3, CellType::ORIGIN);
    g.set_val(9, 2, 9, CellType::ORIGIN);
    g.set_val(9, 3, 2, CellType::ORIGIN);
    g.set_val(9, 4, 6, CellType::ORIGIN);
    g.set_val(9, 5, 5, CellType::ORIGIN);
    g.set_val(9, 6, 4, CellType::ORIGIN);
    g.set_val(9, 7, 8, CellType::ORIGIN);
    g.set_val(9, 8, 1, CellType::ORIGIN);
    g.set_val(9, 9, 7, CellType::ORIGIN);

    let mut l4 = ResolverLvl4::new(false);
    //while not finish we resolve nothing
    assert_eq!(false,l4.resolve(&mut g));
}
