struct Pessoa {
    nome: String,
    idade: i32,
    telefone:i32
}
struct Node{
    dado: Pessoa,
    proximo: Option<Box<Node>>

}
struct List
{head:Option<Box<Node>>

}
impl List {
    fn List_Search(&self,k:i32)-> Option<&Node>  {
    let mut now= &self.head;
    while  let Some(node) =now {
      if node.dado.idade == k{
        return Some(node);
      }
     now= &node.proximo;
    }
    None
}
}

fn main() {
    println!("Hello, world!");
}
