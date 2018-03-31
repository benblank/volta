initSidebarItems({"mod":[["source","Defines a helper macro `define_source_trait!`, for both the `tarball` and `zip` modules to be able to define their own `Source` trait."],["tarball","Provides types and functions for fetching and unpacking a Node installation tarball in Unix operating systems."],["zip","Provides types and functions for fetching and unpacking a Node installation zip file in Windows operating systems."]],"struct":[["Archive","A Node installation tarball."],["Cached","A data source for a Node tarball that has been cached to the filesystem."],["HttpError",""],["Remote","A data source for fetching a Node tarball from a remote server."]],"trait":[["Source","A data source for fetching a Node archive. In Unix operating systems, this is required to implement `Read`. In Windows, this trait extends both `Read` and `Seek` so as to be able to traverse the contents of zip archives."]]});