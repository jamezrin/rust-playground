use std::{env, fs, thread};
use std::io::{BufWriter, Error, Write};

const THING: u32 = 0xf09f8c88;

const THING_COUNT: u64 = u64::pow(10, 8);

const THING_COUNT_FLUSH_THRESHOLD: u32 = u32::pow(10, 3);

const THING_FILE_CHUNKS: u32 = 5;

fn write_chunk(out_path: &std::path::Path) -> Result<(), Error> {
    let file = fs::File::create(out_path)?;
    let mut w = BufWriter::new(&file);

    for i in 0..THING_COUNT {
        let thing_bytes = &(THING.to_be_bytes());
        w.write_all(thing_bytes)?;

        if i % THING_COUNT_FLUSH_THRESHOLD as u64 == 0 {
            w.flush().expect("Flushing buffer failed");
        }
    }

    let metadata = file.metadata()?;
    println!(
        "File '{}' written, size: {} MB",
        out_path.file_name().unwrap().to_str().unwrap(),
        metadata.len() / (1024 * 1024)
    );

    Ok(())
}

fn main() -> Result<(), Error> {
    let out_path = env::current_dir()?
        .join("out");

    if fs::remove_dir_all(&out_path).is_ok() {
        println!("Deleted output directory");
    }

    fs::create_dir_all(&out_path)
        .expect("Failed to create output directory");

    println!("Writing {} file chunks", THING_FILE_CHUNKS);
    thread::scope(|scope| {
        for i in 0..THING_FILE_CHUNKS {
            let chunk_name = format!("chunk-{}", i);
            let file_path = out_path.clone().join(chunk_name.clone());

            scope.spawn(move || {
                match write_chunk(&file_path) {
                    Ok(_) => println!("Successfully wrote chunk {}", chunk_name),
                    Err(_) => eprintln!("Failed to write chunk {}", chunk_name)
                }
            });
        }
    });

    Ok(())
}
