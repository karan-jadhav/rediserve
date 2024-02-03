// This file re-exports route handlers from different modules, making them accessible in main.rs.
pub mod pipeline_route;
pub mod redis_route;

pub use pipeline_route::pipeline_routes;
pub use redis_route::redis_routes;
