fn main() {
    let a: &[u8] =
    b"
        What memory model Rust has? Is it single-threaded or multi-threaded? Is it synchronous or asynchronous? What are the memory layouts of Box and Vector? What are a heap and a stack? Where, but on heap and stack data could live in RAM?
        What runtime Rust has? Does it use a GC (garbage collector)?
        What is special about slice? What is the layout of Rust standard data types? Difference between fat and thin pointers?
        Why does Rust have &str and String types? How do they differ? When should you use them? Why str slice coexists with slice? What is the difference between String and Vec?
        What static typing means? What are the benefits of using it? Weak vs strong typing? Implicit vs explicit typing?
        What are generics and parametric polymorphism? Which problems do they solve?
        What are nominative typing and structural typing? What is the difference?
        What are traits? How are they used? How do they compare to interfaces? What are auto trait and blanket impl? Uncovered type? What are marker traits?
        What are static and dynamic dispatches? Which should you use, and when? What is monomorphization?
        What are a crate, a module, and a package in Rust? How do they differ? How are they used? What is a workspace?
        What is cloning? What is copying? How do they compare? What is trait Drop for? What is special about this trait?
        What is immutability? What is the benefit of using it? What is the difference between immutability and const?
        What are move semantics? What are borrowing rules? What is the benefit of using them?
        What is RAII? How is it implemented in Rust? What is the benefit of using it?
        What are lifetimes? Which problems do they solve? Which benefits do they provide?
        What is an iterator? What is a collection? How do they differ? How are they used?
        What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
        How code is tested in Rust? Where should you put tests and why?
        Is Rust an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance? Is Rust a functional language? What variance rules does Rust have?
    ";
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
