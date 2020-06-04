# diesel_test
Just a minimal code example to replicate a problem I am having with diesel code re-use.

## Situation

I wish to build multiple applications that interact with the same database. So I would like to make a lib crate for diesel database stuff that I can re-use in other crates. This is what resides in `dbstuff`.

In `dbstuff` there is a test application that works successfully. Enter that directory and issue `cargo run --bin test_main`

But I have been unable to get a working application in the binary crate in `appstuff` which references the lib crate. `main.rs` there matches the same code run from within `dbstuff`. Try `cargo run` within that directory.

## Additional details

I am working with a mysql database, so the migrations in dbstuff reflect that. Also will be using numeric/bigdecimal features, which I have included here because prior attempts to resolve have (seemingly) been blocked by that.
