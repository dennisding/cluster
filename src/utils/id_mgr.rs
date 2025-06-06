
use std::cell::{Cell, RefCell};

type IdType = u32;

#[derive(Debug)]
pub struct IdBlock{
	start: Cell<IdType>,
	end: IdType,
}

impl IdBlock{
	pub fn new(start: IdType, size: IdType) -> Self{
		return IdBlock{
			start: Cell::new(start),
			end : start + size,
		}
	}

	pub fn alloc(&self) -> Option<IdType>{
		let start = self.start.get();
		if start >= self.end {
			return None;
		}

		return Some(self.start.replace(start + 1));
	}
}

pub struct IdAllocator{
	start: Cell<IdType>,
	block_size: u32,
}

impl IdAllocator{
	pub fn new() -> Self{
		let start: IdType = 100;
		let block_size: u32 = 1000;
		return IdAllocator{
			start: Cell::new(start),
			block_size
		}
	}

	pub fn alloc(&self) -> IdType {
		return self.start.replace(self.start.get() + 1)
	}

	pub fn alloc_block(&self) -> IdBlock {
		let start = self.start.replace(self.start.get() + self.block_size);

		return IdBlock::new(start, self.block_size);
	}
}

pub struct IdMgr {
	blocks: RefCell<Vec<IdBlock>>,
}

impl IdMgr {
	pub fn new() -> Self {
		return IdMgr {
			blocks: RefCell::new(Vec::new())
		};
	}

	pub fn fill_id_block(&self, block: IdBlock) {
		let mut blocks = self.blocks.borrow_mut();
		blocks.push(block);
	}

	pub fn alloc(&self) -> Option<IdType> {
		let blocks = self.blocks.borrow_mut();
		
		let block = &blocks[0];
		return block.alloc();
	}
}