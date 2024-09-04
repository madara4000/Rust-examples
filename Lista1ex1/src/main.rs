use std::io; 
fn quadrado(a: u32)->u32{
 
a*a 
}
fn dividepor5(a: u32)-> f32{
    a as f32/5.0
}
fn somatres(a:u32,b:u32,c:u32) -> u32{
    a+b+c
}

fn main() { 
    let mut escolha = String::new();
  
    println!("MENU:\n 1.eleva o numero ao quadrado \n 2. divide por 5 \n soma 3 valores ");
    io::stdin()
        .read_line(&mut escolha)
        .expect("falha ao ler a entrada");
        let opcao: u32 = match escolha.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido!");
            return;
        }
    };
    match opcao {
        1 => { let mut numero = String::new();
        println!("entre com um numero inteiro:");
        io::stdin()
           .read_line(&mut numero)
           .expect("falha ao ler entrada");
            let numero1:u32 = match numero.trim().parse(){
            Ok(num) => num,
            Err(_)=>   {
                println!( "Por favor, digite um numero valido!");
                return;}
                                                          };
                println!("O quadrado de {} é {}",numero1 , quadrado(numero1));
            }
        2=> {
            let mut numero = String::new();
            println!("entre com um numero inteiro:");
                io::stdin()
                .read_line(&mut numero)
                .expect("falha ao ler entrada");
            let numero1:u32 = match numero.trim().parse(){
                Ok(num) => num,
                Err(_)=>{
                println!( "Por favor, digite um numero valido!");
                return; }
            };
            println!("O 1/5 de{} é {}",numero1,dividepor5(numero1));
        }      
           3=> {
            let mut numero1 = String::new();
            let mut numero2 = String::new();
            let mut numero3 = String::new();
            println!("Entre com o primeiro numero:");
            io::stdin().read_line(&mut numero1).expect("Falha ao ler a entrada");
            println!("Entre com o segundo numero:");
            io::stdin().read_line(&mut numero2).expect("Falha ao ler a entrada");
            println!("Entre com o terceiro numero:");
            io::stdin().read_line(&mut numero3).expect("Falha ao ler a entrada");  
            let n1:u32 = numero1.trim().parse().expect("Por favor digite um numero valido");
            let n2:u32 = numero1.trim().parse().expect("Por favor digite um numero valido");
            let n3:u32 = numero1.trim().parse().expect("Por favor digite um numero valido");
             println!("A soma de {} + {} + {} é {} ", n1,n2,n3, somatres(n1,n2,n3));
           }
        _=> {
            println!("Opção invalida");
        }
        }
    }





