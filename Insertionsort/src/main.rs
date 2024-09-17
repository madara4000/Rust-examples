fn insertion_sortcres(list:&mut Vec<i32>) 
{   let len = list.len();
    
    for element in 1..len{
        let  key=list[element];
        let mut i = (element as isize) - 1;
        while i >=0 &&list[i as usize] >key {
            list[(i + 1) as usize]=list[(i) as usize];
            i=i-1;
      
        }
        list[(i + 1) as usize]=key;
    }








}

fn insertion_sortdec(list:&mut Vec<i32>) 
{   let len = list.len();
    
    for element in 1..len{
        let  key=list[element];
        let mut i = (element as isize) - 1;
        while i >=0 &&list[i as usize] <key {
            list[(i + 1) as usize]=list[(i) as usize];
            i=i-1;
      
        }
        list[(i + 1) as usize]=key;
    }








}
fn linear_search(index:i32,list:&mut Vec<i32>) -> Option<usize>
{
    for (i, &element) in list.iter().enumerate() {
        if element==index {
            return some(i)
        }
        
    }
 None
}






fn main() {
    let mut vector: Vec<i32> = Vec :: new();
    vector.push(9);
    vector.push(19);
    vector.push(1);
    vector.push(5);
    vector.push(13);
    vector.push(22);
    insertion_sortcres(&mut vector);  
    println!("Vetor ordenado: {:?}", vector); 
    insertion_sortdec(&mut vector);  
    println!("Vetor ordenado: {:?}", vector); 
}
