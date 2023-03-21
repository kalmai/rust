fn main() {
    // one line comment
    /* block comment */
    println!("hello world");

    // seems like most standard library functions have a bang operator after each
    // for some reason using only `#print!` yeilds a '%' at the end of the string :thinking:
    print!("'{}' was interpolated\n", "hank hill");

    // println("without bang operator")
    // seems like you need the bang operator to invoke a macro/function/method, seem to be
    // interchangable, seems like the language has a set line lenth too. cool.
    println!("{0}: first arg, {1} second | positional args", "pro", "pane");

    println!("{song}: {artist} | interpolation with named args", song="long way to happy", artist="p!nk");
}
