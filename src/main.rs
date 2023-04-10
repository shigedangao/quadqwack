use anyhow::Result;
use qtree::QTree;
use rand::Rng;
use rect::Rect;

mod qtree;
mod rect;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut quadtree = QTree::new(0, Rect::new(0, 0, 600, 600));

    for _ in 0..500 {
        quadtree.insert(Rect::new(
            rng.gen_range(0..quadtree.bounds.w - 32),
            rng.gen_range(0..quadtree.bounds.h - 32),
            rng.gen_range(4..32),
            rng.gen_range(4..32),
        ))?;
    }

    let json = serde_json::to_string(&quadtree)?;

    // write to a file the quadtree result
    std::fs::write("output.json", json)?;

    Ok(())
}
