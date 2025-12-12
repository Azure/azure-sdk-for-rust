use std::slice;

use bytes::BytesMut;
use futures::{AsyncRead, AsyncReadExt};

pub(crate) trait AsyncReadTestExt {
    async fn read_into_spare_capacity(
        &mut self,
        buffer: &mut BytesMut,
    ) -> futures::io::Result<usize>;
}

impl<Stream: AsyncRead + Unpin> AsyncReadTestExt for Stream {
    async fn read_into_spare_capacity(
        &mut self,
        buffer: &mut BytesMut,
    ) -> futures::io::Result<usize> {
        let spare_capacity = buffer.spare_capacity_mut();
        let spare_capacity = unsafe {
            // spare_capacity_mut() gives us the known remaining capacity of BytesMut.
            // Those bytes are valid reserved memory but have had no values written
            // to them. Those are the exact bytes we want to write into.
            // MaybeUninit<u8> can be safely cast into u8, and so this pointer cast
            // is safe. Since the spare capacity length is safely known, we can
            // provide those to from_raw_parts without worry.
            slice::from_raw_parts_mut(spare_capacity.as_mut_ptr() as *mut u8, spare_capacity.len())
        };
        let bytes_read = self.read(spare_capacity).await?;
        // read() wrote bytes_read-many bytes into the spare capacity.
        // those values are therefore initialized and we can add them to
        // the existing buffer length
        unsafe { buffer.set_len(buffer.len() + bytes_read) };
        Ok(bytes_read)
    }
}
