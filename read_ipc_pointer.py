import ctypes
import pyarrow.ipc as ipc
import io

# Load Rust shared library
lib = ctypes.cdll.LoadLibrary("./target/release/libpython_rust_ipc.dylib")

# Define return types
lib.export_arrow_ipc.argtypes = [ctypes.POINTER(ctypes.POINTER(ctypes.c_ubyte)), ctypes.POINTER(ctypes.c_size_t)]
lib.export_arrow_ipc.restype = ctypes.c_int

# Call the function
ptr = ctypes.POINTER(ctypes.c_ubyte)()
length = ctypes.c_size_t()

ret = lib.export_arrow_ipc(ctypes.byref(ptr), ctypes.byref(length))

# Convert raw pointer + length to bytes
buffer = ctypes.string_at(ptr, length.value)

# Wrap in BytesIO and read with pyarrow
source = io.BytesIO(buffer)
reader = ipc.open_stream(source)
batch = reader.read_next_batch()

print("Data from Rust pointer via Arrow IPC:")
print(batch)