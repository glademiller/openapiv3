mod components;
mod contact;
mod external_documentation;
mod header;
mod info;
mod license;
mod link;
mod media_type;
mod openapi;
mod operation;
mod parameter;
mod paths;
mod reference;
mod request_body;
mod responses;
mod security_requirement;
mod server;
mod server_variable;
mod tag;

pub use self::components::*;
pub use self::contact::*;
pub use self::external_documentation::*;
pub use self::header::*;
pub use self::info::*;
pub use self::license::*;
pub use self::link::*;
pub use self::media_type::*;
pub use self::openapi::*;
pub use self::operation::*;
pub use self::parameter::*;
pub use self::paths::*;
pub use self::reference::*;
pub use self::request_body::*;
pub use self::responses::*;
pub use self::security_requirement::*;
pub use self::server::*;
pub use self::server_variable::*;
pub use self::tag::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
