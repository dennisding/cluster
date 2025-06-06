
// mod network;
use cluster::network;
use cluster::utils::id_mgr::{IdMgr, IdAllocator};

fn main() {
	network::service::say_hello();
	println!("Hello, world! cluster");

//	let mgr = IdMgr.new();
	let allocator = IdAllocator::new();
	// let block = allocator.alloc_block();
	
	// mgr.add_id_block(block);

	// let id = mgr.alloc();

	println!("alloc id:{}", allocator.alloc());
	println!("alloc id:{}", allocator.alloc());
	println!("alloc id:{}", allocator.alloc());
	println!("alloc id block:{:?}", allocator.alloc_block());
	println!("alloc id:{}", allocator.alloc());

	let block = allocator.alloc_block();
	let mgr = IdMgr::new();
	mgr.fill_id_block(block);

	if let Some(id) = mgr.alloc() {
		println!("mgr alloc:{}", id);
	}
	println!("mgr alloc:{}", mgr.alloc().unwrap());
	println!("mgr alloc:{}", mgr.alloc().unwrap());
	println!("mgr alloc:{}", mgr.alloc().unwrap());
}
