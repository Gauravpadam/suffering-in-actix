use actix_web::{web};
use actix_web_lab::middleware::from_fn;
use super::{handlers::post_handler, middlewares};



pub fn config(config: &mut web::ServiceConfig){
    config
    .service(
        web::scope("secure/post")
        .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
        .service(post_handler::create_post)
        .service(post_handler::my_posts)
    )
    .service(
        web::scope("/post")
        .service(post_handler::all_posts)
        .service(post_handler::one_posts)
    );
}