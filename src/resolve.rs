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


pub fn resolve_lvl2(g:&mut Grid) -> bool
{
    false
}

/*

resolve lvl 2:
Si 1 valeure  trouvée dans 4 carrés de côtés alors on peut trouver où est cette même valeure dans le carré courant
1/ trouve un carré
2/ trouver une valeur non résolue
3/ déterminer les autres carrés
     NW => N NE SW W
     N =>  C S NW NE
     NE => NW N E SE
     W => NW SW C E
     C => N S E W
     E => NE SE W C
     SW => W NW S SE
     S => SW SE C N
     SE => S SW E NE
=> methode a ajouter a Accessor ou cardinal
4/ vérifier si valeur trouvée dans les 4 autres carrés
5/ si oui trouver les 2 lignes et les 2 colonnes
6/ déterminer ligne et colonne restante dans le carré
=> méthode a ajouter a Accessor ou cardinal 
=> trouver cellule de ligne/colonne => set val

boucler tant qu'on trouve des choses
=> en fin de resolve lvl 2 si changement alors refaire resolve 1 puis 2
*/