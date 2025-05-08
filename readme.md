### Sharing Arrow Array between Rust and Python in the same process
Experiment for sharing block of data stored as Apache Arrow cross boundaries within the same process
Rust native code generates a data a send a pointer to data in memory to Python caller

Build Rust application
```
cargo build --release
```
Run Python app that calls Rust method as obtains the response as a pointer to data stored as Arrow and outputs
```
python read_ipc_pointer.py

Data from Rust pointer via Arrow IPC:
pyarrow.RecordBatch
nums: int32 not null
----
nums: [10,20,30]

Process finished with exit code 0
```