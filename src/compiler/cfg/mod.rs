pub mod block;
pub mod inst;

use std::collections::HashMap;
use std::fs::File;
use std::fmt::Error;
use self::block::*;

/// Representation of a control flow graph
#[derive(Debug)]
pub struct CFG {
    /// A map of each CFG block where the block label is the key
    block_map: HashMap<String, Block>,
    /// A mapping of block labels representing an adjacency list
    adjacency_list: HashMap<String, Vec<String>>,
    pub entry_block: String,
    pub exit_block: String
}

impl CFG {
    /// Creates a new control flow graph instance and initializes the entry
    /// and exit nodes as well as the node mappings
    pub fn new() -> CFG {
        let entry_block = Block::new();
        let exit_block = Block::new();
        let mut cfg = CFG {
            block_map: HashMap::new(),
            adjacency_list: HashMap::new(),
            entry_block: entry_block.get_label(),
            exit_block: exit_block.get_label(),
        };

        cfg.block_map.insert(entry_block.get_label(), entry_block);
        cfg.block_map.insert(exit_block.get_label(), exit_block);
        cfg
    }

    /// Adds a block to the CFG block mapping and returns its label
    pub fn add_block(&mut self, block: Block) -> String {
        let label = block.get_label();
        self.block_map.insert(label.clone(), block);
        label
    }

    pub fn output_llvm(&self, f: &mut File) -> Result<(), Error> {
        let mut queue = vec![&self.entry_block];
        let mut visited = vec![&self.entry_block];

        while !queue.is_empty() {
            if let Some(block) = queue.pop() {
                self.block_map.get(block).unwrap().output_llvm(f)?;

                for b in self.adjacency_list.get(block).unwrap().iter() {
                    if !visited.contains(&b) {
                        queue.push(b);
                        visited.push(b);
                    }
                }
            }
        }
        Ok(())
    }
}
