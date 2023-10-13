// task_01 basic Struct
// struct Person{
//     name:String,
//     age:i32
// }

// fn main() {
//     let person_one = Person{
//         name : String::from("parsanta"),
//         age : 16
//     };
//     println!("name os the person is : {}",person_one.name)
// }

// task_02 basic enum
// enum Color{
//     Red,
//     Green,
//     Blue
// }

// fn main(){
//     let color_code = RGB(Color::Red);
//     println!("color code of Red : {:?}",color_code);
// }

// fn RGB(inp:Color)->(i32,i32,i32){
//     match inp {
//         Color::Red=> (255, 0, 0),
//         Color::Blue=>(0,0,255),
//         Color::Green =>(0,128,0),
//     }
// }

// task_03 tuple
// fn main(){
//     let nums= (3,2);
//     let sum = sum_of_tup(nums);
//     print!("sum of vals : {}",sum);
// }
// fn sum_of_tup(vals:(i32,i32))->i32{
//     vals.0+vals.1
// }

// task_04 Option enum
// enum StrOrNum{
//     strVal(String),
//     numVal(i32)
// }

// fn main(){
//     let val1 = Some(StrOrNum::strVal(String::from("hello")));
//     let val2 = Some(StrOrNum::numVal(20));

//     print_some(val2);
// }
// fn print_some(val:Option<StrOrNum>){
//     match val {
//         Some(StrOrNum::numVal(num))=>{
//             println!("the number is {}",num);
//         }
//         Some(StrOrNum::strVal(_))=>{
            
//         }
//         None=>{
//             println!("invalid");
//         }
//     }
// }

// task_05 borrowing
// fn main(){
//     let mut a = String::from("hello,");
//     borrow_str(&mut a);
//     print!("{}",a);
// }

// fn borrow_str(st:&mut String){
//     st.push_str("World");
// }

// task_06 impl prac
// struct Book{
//     title:String
// }

// impl Book{
//     fn new(title:String) -> Self {
//         Self {
//             title,
//         }
//     }

//     fn get_title(&self)->&String{
//         &self.title
//     }
// }
// fn main(){
//     let book1 = Book::new("Mr.Wonka".to_string());
//     let tit_ref=book1.get_title();
//     print!("{tit_ref}");
// }

// task_07 struct+enum
// struct Book {
//     title:String,
//     author:String,
//     pages:i32
// }

// #[derive(Debug)]
// enum Status{
//     Active,
//     Inactive,
//     Suspended
// }

// fn main(){
//     let book1= Book{
//         title:String::from("Wonka"),
//         author:String::from("Mr.Wonka"),
//         pages:50
//     };
//     println!("Book title {}",book1.title);
//     println!("Book status : {:#?}",prac(&book1, Status::Active));
// }

// fn prac(book:&Book,stat:Status)-> (String,Status){
//     match stat{
//         Status::Active => (book.title.clone(),Status::Active),
//         Status::Inactive => (book.title.clone(),Status::Inactive),
//         Status::Suspended => (book.title.clone(),Status::Suspended),
//     }
// }

// task_08 ref of struct
// enum Status{
//     Active,
//     Inactive,
//     Suspended
// }

// task_09
struct Book {
    title:String,
    author:String,
    pages:i32
}
impl Book{
    fn new(title:String,author:String,pages:i32) -> Self {
        Self {
            title,
            author,
            pages
        }
    }
    fn borrow_book(&mut self,s:String){
        self.title.push_str(&s);
    }
        
}
fn main(){
    let mut book1= Book::new("Wonka".to_string(), "MrWonka".to_string(), 32);
    println!("Book title before changes : {}",book1.title);
    book1.borrow_book(" chocolate".to_string());
    println!("Book title after changes : {}",book1.title);


}

// task_10



