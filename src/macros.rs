//! # macros.rs
//!
//! The EBNF rules for Narsese are translated into macros in this file.
//! Each macro defines one non-terminal from Appendix A of the book.

/// ## The <statement> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Primary Copulae
/// - `→` inheritance
/// - `↔` similarity
/// - `⇒` implication
/// - `⇔` equivalence
///
/// #### Secondary Copulae
///
/// - `◦→` instance
/// - `→◦` property
/// - `◦→◦` instance-property
/// - `/⇒` predictive implication
/// - `\⇒` retrospective implication
/// - `|⇒` concurrent implication
/// - `/⇔` predictive equivalence
/// - `|⇔` concurrent equivalence
///
/// #### (optionally) Unary Operators
///
///  - `¬` negation
/// - '⇑' atomic operator
/// - '<term>' Lone Term-Statement
///
/// #### Statement Connectors
///
/// - `∧` conjunction
/// - `∨` disjunction
/// - `,` sequential conjunction
/// - `;` parallel conjunction
///
#[macro_export]
macro_rules! statement
{
    /// Copula-based Statement Cases [Arm 1-12]
    ($left:tt $(→ $right:tt)*) =>
    (
        println!("Registered {:?} inherits {:?}.",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 1: Inheritance Relation

    ($left:tt $(↔ $right:tt)*) =>
    (
        println!("Registered {:?} is similar to {:?}.",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 2: Similarity Relation

    ($left:tt $(⇒ $right:tt)*) =>
    (
        println!("Registered {:?} is implied by {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 3: Implication Relation

    ($left:tt $(⇔ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 4: Equivalence Relation

    ($left:tt $(◦→ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 5: Instance Relation

    ($left:tt $(→◦ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 6: Property Relation

    ($left:tt $(◦→◦ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 7: Instance-Property Relation

    ($left:tt $(/⇒ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 8: Predictive Implication Relation

    ($left:tt $(\⇒ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 9: Retrospective Implication Relation

    ($left:tt $(|⇒ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 10: Concurrent Implication Relation

    ($left:tt $(/⇔ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 11: Predictive Equivalence Relation

    ($left:tt $(|⇔ $right:tt) =>
    (
        println!("Registered {:?} is equivalent to {:?}",
                stringify!($left),
                stringify!($($right)))
    ); /// Arm 12: Concurrent Equivalence Relation

    /// Non-Copula-Based Statement Cases [Arm 13]
    ($($onlyterms:tt)*) =>
    (
        term!($($onlyterms)*);
    ); /// Arm 13: Only Terms

    /// Unary Copula-Based Statement Cases [Arm 14]
    (¬ $term:tt) =>
    (
        println!("Registered {:?} is negated",
                stringify!($term))
    ); /// Arm 14: Unary Operator Statement

    /// Ternary Copula-Based Statement Cases [Arm 15-18]
    (∧ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 15: Conjunction Connector

    (∧ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 16: Disjunction Connector

    (∧ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 17: Sequential Conjunction Connector

    (∧ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 18: Parallel Conjunction Connector

    (⇑ $first:tt $($second:tt)*) =>
    (
        println!("Registered {:?} is in conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 19: Atomic Operation Connector
}
