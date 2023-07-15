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

    // seems like this is acceptable multiline formatting as well afaik?
    println!(
        "{song}: {artist} | interpolation with named args",
        song="long way to happy",
        artist="p!nk"
    );

    println!("{string:>100}", string="justified to the right by 100");
    println!("{:$<width$} padding around this string with $", "doggy", width=20);
    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    println!("Pi is roughly {pi:.precision$}", pi=3.141592, precision=3)
}
