# tuna-file

Allows hot-reloading of Tuna variables from a file. Uses a file-watcher under the hood to watch a file and set all variables from the file.

Note that to get correctly apply the values already in the file at startup you'll have to register variables before creating the file watcher.

``` rust
extern crate tuna;
use std::time::Duration;

#[tuna::tuna]
mod logging {
	pub(super) const ENABLE: bool = false;
}

fn main() {
    logging::register();
	
    let _watcher = tuna_file::open("variables.toml".into(), Duration::from_secs(1)).expect("success");
	/* variables.toml
	[logging]
	ENABLE = true  # enable logging
	*/
	
	assert!(logging::ENABLE.read(), "should be set from file");
}
```

