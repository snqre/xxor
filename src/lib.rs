#![no_std]

use core::fmt;

pub use XOR::*;

/// A type that represents two possible successful outcomes where the types may be different.
/// 
/// This enum is useful when you need to handle two different types that represent
/// successful outcomes in a computation. Unlike `Option` (which can represent both
/// presence and absence of a value) or `Result` (which is for success or failure),
/// `XOR` represents a scenario where one of two distinct successful outcomes can occur. 
/// It can be particularly useful when you don't want to define custom enums for small, 
/// specific use cases with different types.  
/// 
/// # Examples
/// ```rust
/// use xor::*;
/// 
/// let age: XOR<u8, u16> = This(42);
/// assert!(age.is_this());
/// assert!(!age.is_that());
/// assert_eq!(age.this(), Some(42));
/// ```
/// 
/// `XOR` provides several utility methods for mapping, unwrapping, and inspecting
/// the variant it contains.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum XOR<This, That> {
    This(This),
    That(That)
}

impl<This, That> XOR<This, That> {
    pub fn is_this(&self) -> bool {
        matches!(self, This(_))
    }

    pub fn is_that(&self) -> bool {
        matches!(self, That(_))
    }

    pub fn this(self) -> Option<This> {
        if let This(data) = self {
            return Some(data)
        }
        None
    }

    pub fn that(self) -> Option<That> {
        if let That(data) = self {
            return Some(data)
        }
        None
    }

    pub fn map_this<Hook, NewThis>(&self, hook: Hook) -> XOR<NewThis, That> 
    where
        Hook: FnOnce(&This) -> NewThis,
        That: Clone {
        match self {
            This(data) => This(hook(data)),
            That(data) => {
                let data: That = data.clone();
                That(data)
            }
        }
    }

    pub fn map_that<Hook, NewThat>(&self, hook: Hook) -> XOR<This, NewThat> 
    where
        Hook: FnOnce(&That) -> NewThat,
        This: Clone {
        match self {
            This(data) => {
                let data: This = data.clone();
                This(data)
            },
            That(data) => That(hook(data))
        }
    }

    pub fn as_ref(&self) -> XOR<&This, &That> {
        match self {
            This(data) => This(data),
            That(data) => That(data)
        }
    }

    pub fn unwrap_this(self) -> This {
        if let This(data) = self {
            return data
        }
        panic!("Tried to unwrap `This` variant, but it was `That`.")
    }

    pub fn unwrap_that(self) -> That {
        if let That(data) = self {
            return data
        }
        panic!("Tried to unwrap `That` variant, but it was `This`.")
    }
}

impl<This: fmt::Display, That: fmt::Display> fmt::Display for XOR<This, That> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            This(data) => write!(f, "This({})", data),
            That(data) => write!(f, "That({})", data)
        }
    }
}