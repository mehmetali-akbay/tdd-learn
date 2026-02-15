// ============================================
// Level 5, Project 3: Macros — macro_rules!
// Learn: Declarative macros, patterns, repetitions, TT munching
// ============================================

// ============================================
// Topic 1: Basic Macro Patterns
// Learn: Simple matching, expression macros
// ============================================

/// A macro that creates a HashMap from key-value pairs
#[macro_export]
macro_rules! hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    }};
}

/// A macro that creates a Vec with push syntax
#[macro_export]
macro_rules! vec_of {
    ($($item:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($item);)*
        v
    }};
}

/// A macro that returns the min of N expressions
#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {
        {
            let x = $x;
            let rest = min!($($rest),+);
            if x < rest { x } else { rest }
        }
    };
}

/// A macro that returns the max of N expressions
#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {
        {
            let x = $x;
            let rest = max!($($rest),+);
            if x > rest { x } else { rest }
        }
    };
}

// ============================================
// Topic 2: Repetition Patterns
// Learn: *, +, ?, nested repetitions
// ============================================

/// A macro that counts its arguments
#[macro_export]
macro_rules! count {
    () => (0usize);
    ($x:expr $(, $rest:expr)*) => (1usize + count!($($rest),*));
}

/// A macro that sums its arguments
#[macro_export]
macro_rules! sum {
    () => (0);
    ($x:expr $(, $rest:expr)*) => ($x + sum!($($rest),*));
}

/// Average of arguments (as f64)
#[macro_export]
macro_rules! avg {
    ($($x:expr),+ $(,)?) => {{
        let total: f64 = 0.0 $(+ $x as f64)+;
        let count: f64 = 0.0 $(+ { let _ = $x; 1.0 })+;
        total / count
    }};
}

/// A macro that creates a tuple struct with a named constructor
#[macro_export]
macro_rules! newtype {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name(pub $inner);

        impl $name {
            pub fn new(value: $inner) -> Self {
                $name(value)
            }

            pub fn value(&self) -> &$inner {
                &self.0
            }
        }
    };
}

// ============================================
// Topic 3: Pattern Matching in Macros
// Learn: Multiple macro arms, different input patterns
// ============================================

/// Conditional execution macro: `when!(condition => expression)`
#[macro_export]
macro_rules! when {
    ($cond:expr => $body:expr) => {
        if $cond {
            Some($body)
        } else {
            None
        }
    };
    ($cond:expr => $body:expr, else => $else_body:expr) => {
        if $cond {
            $body
        } else {
            $else_body
        }
    };
}

/// A string builder macro: `str_join!("a", "b", "c") => "abc"`
#[macro_export]
macro_rules! str_join {
    ($($s:expr),* $(,)?) => {{
        let mut result = String::new();
        $(result.push_str($s);)*
        result
    }};
    ($sep:expr; $($s:expr),+ $(,)?) => {{
        let parts: Vec<&str> = vec![$($s),+];
        parts.join($sep)
    }};
}

/// An assert_between macro
#[macro_export]
macro_rules! assert_between {
    ($val:expr, $low:expr, $high:expr) => {
        let val = $val;
        assert!(
            ($low..=$high).contains(&val),
            "{} is not between {} and {}",
            val,
            $low,
            $high
        );
    };
}

// ============================================
// Topic 4: Struct & Enum Generation
// Learn: Using macros to generate types and impls
// ============================================

/// Generate an enum with Display and from_str
#[macro_export]
macro_rules! string_enum {
    ($name:ident { $($variant:ident => $str:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name {
            $($variant),+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $str)),+
                }
            }
        }

        impl $name {
            pub fn from_str_custom(s: &str) -> Option<Self> {
                match s {
                    $($str => Some($name::$variant)),+,
                    _ => None,
                }
            }

            pub fn variants() -> Vec<&'static str> {
                vec![$($str),+]
            }
        }
    };
}

/// Generate a builder pattern
#[macro_export]
macro_rules! builder {
    ($name:ident { $($field:ident : $ty:ty = $default:expr),+ $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $ty),+
        }

        impl $name {
            pub fn builder() -> Self {
                $name {
                    $($field: $default),+
                }
            }

            $(
                pub fn $field(mut self, value: $ty) -> Self {
                    self.$field = value;
                    self
                }
            )+
        }
    };
}

// ============================================
// Topic 5: Utility Macros
// Learn: Debug helpers, timing, error handling
// ============================================

/// A macro that wraps a block and returns its execution time in microseconds
#[macro_export]
macro_rules! timed {
    ($body:expr) => {{
        let start = std::time::Instant::now();
        let result = $body;
        let elapsed = start.elapsed().as_micros();
        (result, elapsed)
    }};
}

/// A macro that retries an expression N times
#[macro_export]
macro_rules! retry {
    ($n:expr, $body:expr) => {{
        let mut last_err = None;
        for _ in 0..$n {
            match $body {
                Ok(val) => return Ok(val),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap())
    }};
}

/// A macro that creates a match expression with a default arm
#[macro_export]
macro_rules! match_or {
    ($expr:expr, $default:expr, $($pattern:pat => $result:expr),+ $(,)?) => {
        match $expr {
            $($pattern => $result),+,
            _ => $default,
        }
    };
}

// ============================================
// Topic 6: Advanced — Recursive Macros
// Learn: TT munching, recursive expansion
// ============================================

/// Reverse a list of expressions
#[macro_export]
macro_rules! reverse_vec {
    // Public entry point
    ($($item:expr),* $(,)?) => {{
        let mut v = vec![$($item),*];
        v.reverse();
        v
    }};
}

/// Create nested pairs: `nest!(1, 2, 3)` => (1, (2, (3, ())))
#[macro_export]
macro_rules! nest {
    () => { () };
    ($first:expr $(, $rest:expr)*) => {
        ($first, nest!($($rest),*))
    };
}

/// Chain method calls: `chain_calls!(obj, method1(), method2(arg))`
#[macro_export]
macro_rules! chain_calls {
    ($obj:expr, $method:ident($($arg:expr),*)) => {
        $obj.$method($($arg),*)
    };
    ($obj:expr, $method:ident($($arg:expr),*), $($rest:ident($($rarg:expr),*)),+) => {
        chain_calls!($obj.$method($($arg),*), $($rest($($rarg),*)),+)
    };
}

/// Compute fibonacci at compile time (for small values)
#[macro_export]
macro_rules! const_add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}
