// declare 3 submodules of module http: method, request, response
mod method;
mod request;
mod response;

/*
`use` is used to import an item into the current module (scope)
`pub use` allows us to not only import but re-export the item
so that it can be used by other modules/crates (similar to modules.expor = ... in JS)
https://stackoverflow.com/questions/69275034/what-is-the-difference-between-use-and-pub-use
*/
pub use method::Method;
pub use request::Request;