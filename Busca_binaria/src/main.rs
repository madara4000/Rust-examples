fn busca_binaria(item:i32,lista:&Vec<i32>) -> Option<usize>
{ 
    let mut baixo:usize = 0;
    let mut alto:usize = lista.len()-1;
    while baixo<= alto{
        let  meio:usize = (baixo+alto)/2;
        let  chute = lista[meio];
        if chute< item{
            baixo = meio+1;
        } else if chute>item{
            alto = meio -1;

        } else{
            return Some (meio);

        }
        
    }
    None

}

fn main() {
    let mut vetor: Vec<i32> = Vec:: new();
    vetor.push(3);
    vetor.push(5);
    vetor.push(15);
    vetor.push(25);
    vetor.push(50);
    vetor.push(65);
    let  num:i32=25;
    match busca_binaria(num,&vetor){
        Some(posição)=>println!("valor {}  está na posição {}",num,posição),
        None=>println!("valor não encontrado"),
}
}
