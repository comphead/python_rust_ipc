use std::sync::Arc;
use arrow::array::{Int32Array, ArrayRef};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::ipc::writer::StreamWriter;
use arrow::record_batch::RecordBatch;

#[no_mangle]
pub extern "C" fn export_arrow_ipc(ptr_out: *mut *const u8, len_out: *mut usize) -> i32 {
    // Arrow schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("nums", DataType::Int32, false),
    ]));

    // Arrow Data
    let array: ArrayRef = Arc::new(Int32Array::from(vec![10, 20, 30]));
    let batch = RecordBatch::try_new(schema.clone(), vec![array]).unwrap();

    // Output IPC buffer
    let mut buffer = Vec::new();
    {
        let mut writer = StreamWriter::try_new(&mut buffer, &schema).unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();
    }

    let len = buffer.len();
    let ptr = buffer.as_ptr();

    std::mem::forget(buffer); // Don't free buffer; Python will need to copy or clean

    unsafe {
        *ptr_out = ptr;
        *len_out = len;
    }

    0 // success
}
