#[diagnostic::on_unimplemented(
message = "Here is my message for `ImportantTrait<{A}>` is not implemented for `{Self}` type!",
label = "Missing implementation for `ImportantTrait<{A}>`",
note = "You need to implement MyTrait for this type to use it here.",
note = "Note 2"
)]
trait ImportantTrait<A> {}

fn use_my_trait(_: impl ImportantTrait<i32>) {}

fn main() {
    use_my_trait(String::new());
}