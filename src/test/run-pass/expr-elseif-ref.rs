// Make sure we drop the refs of the temporaries needed to return the
// values from the else if branch
fn main() {
    let y: @uint = @10u;
    let x = if false { y } else if true { y } else { y };
    assert (*y == 10u);
}
