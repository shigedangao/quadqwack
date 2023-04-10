use crate::rect::Rect;
use anyhow::Result;
use serde::Serialize;

// Constant
const MAX_OBJECTS: usize = 4;
const MAX_LEVELS: usize = 4;

/// Implementation based on https://gamedevelopment.tutsplus.com/tutorials/quick-tip-use-quadtrees-to-detect-likely-collisions-in-2d-space--gamedev-374
#[derive(Debug, Clone, Default, Serialize)]
pub struct QTree {
    pub level: usize,
    pub objects: Vec<Rect>,
    pub bounds: Rect,
    pub nodes: Option<Box<[QTree; 4]>>,
}

impl QTree {
    pub fn new(node_level: usize, bounds: Rect) -> Self {
        Self {
            level: node_level,
            bounds,
            objects: Vec::new(),
            nodes: None,
        }
    }

    /// Clear all objects that exists within the quadtree from all nodes
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        // Clear the objects that the quadtree handled
        self.objects.clear();

        if let Some(nodes) = self.nodes.as_mut() {
            for node in nodes.iter_mut() {
                // Clearing each node recursively
                node.clear();
            }
        }
    }

    /// Subdivide the nodes into 4 equal subpart
    pub fn subdivide(&mut self) {
        let (sw, hw) = self.bounds.get_sub_dimensions();
        let (x, y) = self.bounds.get_x_and_y();
        let level = self.level + 1;

        self.nodes = Some(Box::new([
            QTree::new(level, Rect::new(x + sw, y, sw, hw)),
            QTree::new(level, Rect::new(x, y, sw, hw)),
            QTree::new(level, Rect::new(x, y + hw, sw, hw)),
            QTree::new(level, Rect::new(x + sw, y + hw, sw, hw)),
        ]))
    }

    pub fn insert(&mut self, rect: Rect) -> Result<()> {
        if let Some(nodes) = self.nodes.as_mut() {
            let index = self.bounds.contains_rect(&rect);

            if index != -1 {
                return match nodes.get_mut(index as usize) {
                    Some(node) => node.insert(rect),
                    None => Err(anyhow::format_err!(
                        "Unable to insert a node for index of {index}"
                    )),
                };
            }
        }

        self.objects.push(rect);

        if self.objects.len() > MAX_OBJECTS && self.level < MAX_LEVELS {
            // Subdivide the nodes
            if self.nodes.is_none() {
                self.subdivide();
            }

            // Add objects to the corresponding nodes
            for object in self.objects.iter() {
                let index = self.bounds.contains_rect(object);
                if index != -1 {
                    if let Some(nodes) = self.nodes.as_mut() {
                        // Add the nodes recursively
                        match nodes.get_mut(index as usize) {
                            Some(node) => node.insert(object.clone()),
                            None => Err(anyhow::format_err!(
                                "Unable to insert a node for index of {index}"
                            )),
                        }?;
                    }
                }
            }

            self.objects.clear();
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn retrieve(&mut self, obj: &mut Vec<Rect>, rect: Rect) -> Vec<Rect> {
        let idx = self.bounds.contains_rect(&rect);
        if let Some(nodes) = self.nodes.as_mut() {
            if idx != -1 {
                nodes[idx as usize].retrieve(obj, rect);
            }
        }

        obj.append(&mut self.objects);

        obj.to_owned()
    }
}
