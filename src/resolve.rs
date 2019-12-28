use super::grid::*;

pub fn resolve_lvl1(g:&mut Grid) -> bool
{
    //if already resolved...
    if g.is_resolved(){
        return true;
    }
    //else get resolved cells positions
    let mut resolved = g.get_resolved();    
    let mut prev_count;
    loop{
        println!("resolved = {:?}",resolved);
        prev_count = resolved.len();
        for p in resolved{
            g.resolve_lvl1(p);
        }
        resolved = g.get_resolved();    
        //stop when no more cells to resolved
        if resolved.len() == prev_count{
            break;
        }
    }//
    if prev_count == 81{
        return true;
    }
    false
}
