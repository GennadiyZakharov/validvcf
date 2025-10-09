use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn open_maybe_gzipped<P>(path: P) -> io::Result<BufReader<Box<dyn Read>>>
where
    P: AsRef<Path>,
{
    // Determine whether we think the file is gzipped.
    let is_gzip = path
        .as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "gz" | "bgz"))
        .unwrap_or(false);

    // Open the file
    let file = File::open(path)?;

    // Choose the appropriate reader.
    let inner: Box<dyn Read> = if is_gzip {
        // MultiGzDecoder handles normal gzip streams as well as concatenated
        // members (the format used by BGZF).  It also transparently passes
        // through uncompressed data if the file isn’t actually gzipped,
        // but we keep the explicit check for clarity.
        Box::new(flate2::read::MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };

    Ok(BufReader::new(inner))
}
