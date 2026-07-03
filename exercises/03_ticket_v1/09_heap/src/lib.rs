pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24); // solved by own assuming the each String on a 64-bit machine is actually made up of three usize values.
        // pointer(8 bytes), length(8 bytes), capacity(8 bytes) so 24 bytes.
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72); // here by multiplying 24*3 for 3 Strings
    }
}

// demo
//       +---------+--------+----------+
// Stack | pointer | length | capacity | 
//       |  |      |   0    |    5     |
//       +--|------+--------+----------+
//          |
//          |
//          v
//        +---+---+---+---+---+
// Heap:  | ? | ? | ? | ? | ? |
//        +---+---+---+---+---+


//      +---------+--------+----------+
// Stack | pointer | length | capacity |
//       |  |      |   3    |    5     |
//       +--|  ----+--------+----------+
//          |
//          |
//          v
//        +---+---+---+---+---+
// Heap:  | H | e | y | ? | ? |
//        +---+---+---+---+---+
