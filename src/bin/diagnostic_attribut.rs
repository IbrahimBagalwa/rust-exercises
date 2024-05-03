trait BeforeDiagnostic<A> {}

fn use_my_trait(_: impl BeforeDiagnostic<i32>){}
fn main(){
    use_my_trait(String::new());
}