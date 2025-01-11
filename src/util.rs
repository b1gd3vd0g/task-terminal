pub fn print_to_width(output: &str, width: usize) -> Result<(), ()> {
    if output.len() > width {
        // Error!
        let partial_output = &output[..width];
        print!("{partial_output}");
        return Err(());
    }
    print!("{output}");
    for _ in 0..width - output.len() {
        print!(" ");
    }
    Ok(())
}