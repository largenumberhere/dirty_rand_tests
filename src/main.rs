use dirty_rand::DirtyRand;

fn main() {
    //println!("Hello, world!");

    let dirt = DirtyRand{};
    let number = DirtyRand::make_u32();

    println!("{number}");

    let byte = DirtyRand::make_u8();
    println!("{byte}");

    let us = DirtyRand::make_usize();
    println!("{us}");

    for i in 0..10 {
        let n = DirtyRand::make_u32();
        println!("n={n}");
    }
}

#[cfg(test)]
mod tests{
    use std::io::Write;
    use dirty_rand::DirtyRand;

    const ITER_COUNT: usize = 10_000;

    #[test]
    fn spam_u32 () {
        for _ in 0..ITER_COUNT {
            let num = DirtyRand::make_u32();

            // Make sure num isn't optimised away for being unused
            println!("spam_u32: {num}");
        }
    }

    #[test]
    fn spam_threaded_u32() {
        let mut handles: Vec<_> = Vec::new();

        for i in 0.. ITER_COUNT{
            let t = std::thread::spawn(|| {
                let n: u32 = DirtyRand::make_u32();
                n
            });

            handles.push(t);
        }

        // Wait for all to finish and dump out results
        //let mut out = std::io::stdout().lock();
        for h in handles{
            let v = h.join().expect("Thread join error");
            println!("spam_threaded_u32: {}", v);
            //out.write_fmt(format_args!("{}\n", v)).unwrap();
        }


    }

    // #[test]
    // fn


}
