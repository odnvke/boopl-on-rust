// processors/mod.rs
mod else_preprocessor;
mod function_preprocessor;
mod range_expander;
mod parentheses_preprocessor;

// Реэкспортируем
pub use function_preprocessor::expand;
pub use range_expander::expand_ranges;
pub use parentheses_preprocessor::parentheses_process;
pub use else_preprocessor::else_processing;