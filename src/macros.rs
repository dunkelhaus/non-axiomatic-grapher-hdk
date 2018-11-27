//! # macros.rs
//!
//! The EBNF rules for Narsese are translated into macros in this file.
//! Each macro defines one non-terminal from Appendix A of the book.

/// ## The <judgement> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Truth Value Designator
/// - . Truth Value
///
#[macro_export]
macro_rules! judgement
{
    (/⇒ $statement:tt . $truthval:tt) =>
    (
        println!("Registered {:?} as a statement in future tense with truth value {:?}",
                stringify!($statement),
                stringify!($truthval));
    ); /// Arm 1: Future tense judgement

    (\⇒ $statement:tt . $truthval:tt) =>
    (
        println!("Registered {:?} as a statement in past tense with truth value {:?}",
                stringify!($statement),
                stringify!($truthval));
    ); /// Arm 2: Past tense judgement

    (|⇒ $statement:tt . $truthval:tt) =>
    (
        println!("Registered {:?} as a statement in present tense with truth value {:?}",
                stringify!($statement),
                stringify!($truthval));
    ); /// Arm 3: Present tense judgement

    ($statement:tt . $truthval:tt) =>
    (
        println!("Registered {:?} as a statement with truth value {:?}",
                stringify!($statement),
                stringify!($truthval));
    ); /// Arm 4: No tense judgement
}

/// ## The <goal> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Desire Value / Goal Designator
/// - ! Desire Value
///
#[macro_export]
macro_rules! goal
{
    ($statement:tt ! $desireval:tt) =>
    (
        println!("Registered {:?} as a goal with desire value {:?}",
                stringify!($statement),
                stringify!($truthval));
    ); /// Arm 1: Base Goal Rule
}

/// ## The <question> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Desire Value / Goal Designator
/// - ? Question (on truth value)
/// - ¿ Question (on desire value)
///
#[macro_export]
macro_rules! question
{
    (/⇒ $statement:tt ?) =>
    (
        println!("Registered {:?} as a predictive question",
                stringify!($statement));
    ); /// Arm 1: Future tense question

    (\⇒ $statement:tt ?) =>
    (
        println!("Registered {:?} as a retrospective question",
                stringify!($statement));
    ); /// Arm 2: Past tense question

    (|⇒ $statement:tt ?) =>
    (
        println!("Registered {:?} as a current question",
                stringify!($statement));
    ); /// Arm 3: Present tense question

    ($statement:tt ?) =>
    (
        println!("Registered {:?} as a question",
                stringify!($statement));
    ); /// Arm 4: No tense question

    ($statement:tt ¿) =>
    (
        println!("Registered {:?} as a question about the desire value",
                stringify!($statement));
    ); /// Arm 5: Desire value question
}

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
        println!("Registered {:?} is an instance of {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 5: Instance Relation

    ($left:tt $(→◦ $right:tt) =>
    (
        println!("Registered {:?} has a property {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 6: Property Relation

    ($left:tt $(◦→◦ $right:tt) =>
    (
        println!("Registered {:?} is an instance of and has property {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 7: Instance-Property Relation

    ($left:tt $(/⇒ $right:tt) =>
    (
        println!("Registered {:?} predictively implies {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 8: Predictive Implication Relation

    ($left:tt $(\⇒ $right:tt) =>
    (
        println!("Registered {:?} retrospectively implies {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 9: Retrospective Implication Relation

    ($left:tt $(|⇒ $right:tt) =>
    (
        println!("Registered {:?} concurrently implies {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 10: Concurrent Implication Relation

    ($left:tt $(/⇔ $right:tt) =>
    (
        println!("Registered {:?} is predictively equivalent to {:?}",
                    stringify!($left),
                    stringify!($($right)))
    ); /// Arm 11: Predictive Equivalence Relation

    ($left:tt $(|⇔ $right:tt) =>
    (
        println!("Registered {:?} is concurrently equivalent to {:?}",
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

    (∨ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in disjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 16: Disjunction Connector

    (, $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in sequential conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 17: Sequential Conjunction Connector

    (; $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in parallel conjunction with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 18: Parallel Conjunction Connector

    (⇑ $first:tt $($second:tt)*) =>
    (
        println!("Registered {:?} is in an atomic operation with {:?}",
                stringify!($first),
                stringify!($($second)+)
    ); /// Arm 19: Atomic Operation Connector
}

/// ## The <term> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Term Connector
/// - `{}` extensional set
/// - `[]` intensional set
/// - `∩` extensional intersection
/// - `∪` intensional intersection
/// - `−` extensional difference
/// - `⊖` intensional difference
/// - `×` product
/// - `/` extensional image
/// - `\` intensional image
///
#[macro_export]
macro_rules! term
{
    ($word:tt) =>
    (
        println!("Registered {:?} as a lone <word> term",
                stringify!($first));
    ); /// Arm 1: Lone <word> term

    ($($statement:tt)*) =>
    (
        statement!($statement);
        variable!($statement);
    ); /// Arm 2: <statement> & <variable> term

    ( { $($term:tt)+ } ) =>
    (
        println!("Registered {:?} is an extensional set of terms",
                stringify!($first));
    ); /// Arm 3: Extensional Set Connector

    ( [ $($term:tt)+ ] ) =>
    (
        println!("Registered {:?} is an intensional set of terms",
                stringify!($first));
    ); /// Arm 4: Intensional Set Connector

    (∩ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in an extensional intersection operation with {:?}",
                stringify!($first),
                stringify!($($second)+));
    ); /// Arm 5: Extensional Intersection Connector

    (∪ $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in an intensional intersection operation with {:?}",
                stringify!($first),
                stringify!($($second)+));
    ); /// Arm 6: Intensional Intersection Connector

    (− $first:tt $($second:tt)) =>
    (
        println!("Registered {:?} is in an extensional difference operation with {:?}",
                stringify!($first),
                stringify!($($second)+));
    ); /// Arm 7: Extensional Difference Connector

    (⊖ $first:tt $($second:tt)) =>
    (
        println!("Registered {:?} is in an intensional difference operation with {:?}",
                stringify!($first),
                stringify!($($second)+));
    ); /// Arm 8: Intensional Difference Connector

    (× $first:tt $($second:tt)+) =>
    (
        println!("Registered {:?} is in a product operation with {:?}",
                stringify!($first),
                stringify!($($second)+));
    ); /// Arm 9: Product Connector

    (/ $first:tt $($second:tt)* ⋄ $($(third))*) =>
    (
        println!("Registered {:?} and {:?} are in an extensional image operation with {:?}",
                stringify!($first),
                stringify!($($second)*),
                stringify!($($third)*));
    ); /// Arm 10: Extensional Image Connector

    (\ $first:tt $($second:tt)* ⋄ $($(third))*) =>
    (
        println!("Registered {:?} and {:?} are in an intensional image operation with {:?}",
                stringify!($first),
                stringify!($($second)*),
                stringify!($($third)*));
    ); /// Arm 11: Intensional Image Connector
}

/// ## The <independent_variable> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Term Prefix
/// - #
///
#[macro_export]
macro_rules! independent_variable
{
    (#$word:tt) =>
    (
        println!("Registered {:?} as an independent variable",
                stringify!($first));
    ); /// Arm 1: Base independent variable
}

/// ## The <dependent_variable> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Term Prefix
/// - # Variable
/// - () Independent variable
///
#[macro_export]
macro_rules! dependent_variable
{
    (#$word:tt ($var:tt)) =>
    (
        println!("Registered {:?} as a dependent variable on {:?}");
    ); // Arm 1: Base dependent variable
}

/// ## The <independent_variable> non-terminal.
/// ### Copulae/Connectors supported here include:
///
/// #### Term Prefix
/// - ? Query variable
///
#[macro_export]
macro_rules! query_variable
{
    (#[$word:tt]) =>
    (
        println!("Registered {:?} as a query variable",
                stringify!($first));
    ); /// Arm 1: Base query variable
}
