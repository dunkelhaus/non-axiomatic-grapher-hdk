#[macro_export]
macro_rules! relation
{
    ($left:tt $(→ $right:tt)*) =>
    (
        println!("Registered {:?} inherits {:?}.",
                 stringify!($right),
                 stringify!($left)
        ); // Arm 1: Inheritance Relation
    )

    ($($left:tt ↔ $right:tt)*) =>
    (
        println!("Registered {:?} is similar to {:?}.",
                 stringify!($right),
                 stringify!($left)
    ); // Arm 2: Similarity Relation

    ($left:tt $(⇒ $right:tt)*) =>
    (
        println!("Registered {:?} is implied by {:?}",
                 stringify!($right),
                 stringify!($left)
    ); // Arm 3: Implication Relation

    ($left:tt ⇔ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                 stringify!($right),
                 stringify!($left)
    ); // Arm 4: Equivalence Relation
}
