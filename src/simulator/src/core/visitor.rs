use crate::models::*;

pub trait Visitor<T> {
   fn visit(club: &T);
}
