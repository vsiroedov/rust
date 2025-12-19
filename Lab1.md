Lab1

    1. Is Rust single-threaded or multi-threaded? Is it synchronous or asynchronous?
        Rust is multi-threaded, meaning it can run many tasks at the same time. It supports both synchronous and asynchronous programming.
    2. What runtime Rust has? Does it use a GC (garbage collector)?
       Rust has a very small runtime and does not use a garbage collector. It manages memory automatically using a system called "ownership" during compilation.
    3. What static typing means? What are the benefits of using it?
        Static typing means the computer checks the types of your data (like numbers or text) before the program runs. This helps find errors early and makes the program more reliable.
    4. What is immutability? What is the benefit of using it?
        Immutability means that once you create a variable, its value cannot be changed. This makes the code safer and easier to understand because you don't have to worry about data changing unexpectedly.
    5. What are move semantics? What are borrowing rules? What is the benefit of using them?
        Move semantics mean that only one part of the code "owns" a piece of data at a time. Borrowing rules let you use data without owning it, which prevents memory errors without slowing down the program.
    6. What are traits? How are they used? How do they compare to interfaces?
        Traits define shared behavior that different types can use. They are very similar to "interfaces" in other languages like Java or C#.
    7. What are lifetimes? Which problems do they solve?
        Lifetimes are rules that tell the compiler how long a reference stays valid. They prevent "dangling pointers," which happen when a program tries to use data that has already been deleted.
    8. What are macros? Which problems do they solve?
        Macros are a way to write code that generates other code. They help reduce repetitive work and let you do things that normal functions cannot do.
    9. What is the difference between &String and &str types (or between &Vec and &[u8] types)? Difference between fat and thin pointers?
      &String is a reference to a specific string object, while &str is a "slice" that can look at any part of a string. &str is a "fat pointer" because it stores both the memory address and the length of the data.
    10. What are static and dynamic dispatches?
       Static dispatch decides which function to call when the code is compiled, making it very fast. Dynamic dispatch decides which function to call while the program is running, which is more flexible but a bit slower.
