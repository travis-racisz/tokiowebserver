mod about;
pub mod blog;
mod index;
pub use about::get_about;
pub use blog::get_posts;
pub use index::get_index;
