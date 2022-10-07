// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a Self, &'b T);
    //~^ ERROR the parameter type `Self` may not live long enough
    //~| ERROR the parameter type `T` may not live long enough
}

impl<'a, 'b, T, U> MyTrait<'a, 'b, T> for U {
    async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
