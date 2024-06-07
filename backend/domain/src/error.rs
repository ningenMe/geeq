#[derive(Debug)]
pub enum CustomError {
    Unauthenticated,
    Unauthorized,
    InternalServerError,
    DomainModelError,
}
