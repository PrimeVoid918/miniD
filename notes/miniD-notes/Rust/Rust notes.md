```rust 
	// to do localized async 
	fn main() {
	    let future = async {
	        let x = fetch_data().await;
	        println!("Got {}", x);
	    };
	
	    // Must run the future in a runtime
	    tokio::runtime::Runtime::new().unwrap().block_on(future);
	}
}
```