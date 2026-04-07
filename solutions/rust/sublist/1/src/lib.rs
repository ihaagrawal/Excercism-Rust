#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sub(a: &[i32], b: &[i32]) -> bool{
    if a.is_empty(){
        return true;
    }

    if a.len() > b.len(){
        return false;
    }

    b.windows(a.len()).any(|w| w == a)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    }else if is_sub(first_list, second_list){
        Comparison::Sublist
    }else if is_sub(second_list, first_list){
        Comparison::Superlist
    }else{
        Comparison::Unequal
    }
}
