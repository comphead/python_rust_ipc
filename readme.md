### Sharing Arrow Array between Rust and Python in the same process
```
cargo build --release
```

```
python read_ipc_pointer.py

Data from Rust pointer via Arrow IPC:
pyarrow.RecordBatch
nums: int32 not null
----
nums: [10,20,30]

Process finished with exit code 0
```