use core::mem::size_of;
use core::marker::PhantomData;
use core::slice;

#[repr(transparent)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct sptr<T> {
    ptr: usize,
    _type: PhantomData<T>,
}

impl<T> sptr<T> {
    pub unsafe fn from_vaddr(vaddr: usize) -> sptr<T> {
        sptr::<T> {
            ptr: vaddr,
            _type: PhantomData,
        }
    }

    pub unsafe fn cast<H>(self) -> sptr<H> {
        sptr::<H>::from_vaddr(self.ptr)
    }

    pub unsafe fn seek(&mut self, by: isize) {
        self.ptr = self.ptr.wrapping_add(size_of::<T>() * (by as usize));
    }

    pub fn raw(self) -> usize {
        self.ptr
    }
}

extern {
    fn copy64_to_sm(dst: *mut [u64; 8], src: sptr<[u64; 8]>);
    fn copy8_to_sm(dst: *mut u64, src: sptr<u64>);
    fn copy1_to_sm(dst: *mut u8, src: sptr<u8>);

    fn copy64_from_sm(dst: sptr<[u64; 8]>, src: *const [u64; 8]);
    fn copy8_from_sm(dst: sptr<u64>, src: *const u64);
    fn copy1_from_sm(dst: sptr<u8>, src: *const u8);
}

pub fn copy_in<T: Copy>(dst: &mut T, src: sptr<T>)
{
    let dst_raw = dst as *mut T as *mut u8;
    let dst_len = size_of::<T>();
    unsafe {
        let dst_slice = slice::from_raw_parts_mut(dst_raw, dst_len);
        copy_buf_in(dst_slice, src.cast());
    }
}

pub fn copy_out<T: Copy>(dst: sptr<T>, src: &T)
{
    let src_raw = src as *const T as *const u8;
    let src_len = size_of::<T>();
    unsafe {
        let src_slice = slice::from_raw_parts(src_raw, src_len);
        copy_buf_out(dst.cast(), src_slice);
    }
}

pub fn copy_buf_in(mut dst: &mut [u8], mut src: sptr<u8>)
{
    let dst_addr = dst.as_ptr() as usize;
    let src_addr = src.ptr;

    if src_addr % 8 == 0 && dst_addr % 8 == 0 {
        while dst.len() >= 64 {
            unsafe {
                copy64_to_sm(dst.as_mut_ptr() as *mut [u64; 8], src.cast());
                src.seek(64);
            }

            dst = &mut dst[64..];
        }

        while dst.len() >= 8 {
            unsafe {
                copy8_to_sm(dst.as_mut_ptr() as *mut u64, src.cast());
                src.seek(8);
            }

            dst = &mut dst[8..];
        }
    }

    while dst.len() > 0 {
        unsafe {
            copy1_to_sm(dst.as_mut_ptr(), src);
            src.seek(1);
        }

        dst = &mut dst[1..];
    }
}

pub fn copy_buf_out(mut dst: sptr<u8>, mut src: &[u8])
{
    let src_addr = src.as_ptr() as usize;
    let dst_addr = dst.ptr;

    if src_addr % 8 == 0 && dst_addr % 8 == 0 {
        while src.len() >= 64 {
            unsafe {
                copy64_from_sm(dst.cast(), src.as_ptr() as *const [u64; 8]);
                dst.seek(64);
            }

            src = &src[64..];
        }

        while src.len() >= 8 {
            unsafe {
                copy8_from_sm(dst.cast(), src.as_ptr() as *const u64);
                dst.seek(8);
            }

            src = &src[8..];
        }
    }

    while src.len() > 0 {
        unsafe {
            copy1_from_sm(dst, src.as_ptr());
            dst.seek(1);
        }

        src = &src[1..];
    }
}
