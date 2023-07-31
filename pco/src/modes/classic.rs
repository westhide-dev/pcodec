use crate::bin::{BinCompressionInfo, BinDecompressionInfo};
use crate::bit_reader::BitReader;
use crate::bits;
use crate::constants::Weight;
use crate::data_types::UnsignedLike;
use crate::errors::PcoResult;
use crate::modes::ConstMode;

// formula: bin lower + offset
#[derive(Clone, Copy, Debug)]
pub struct ClassicMode;

impl<U: UnsignedLike> ConstMode<U> for ClassicMode {
  type BinOptAccumulator = ();
  fn combine_bin_opt_acc(_bin: &BinCompressionInfo<U>, _acc: &mut Self::BinOptAccumulator) {}

  fn bin_cost(&self, lower: U, upper: U, count: Weight, _acc: &Self::BinOptAccumulator) -> f64 {
    (bits::bits_to_encode_offset(upper - lower) as u64 * count as u64) as f64
  }
  fn fill_optimized_compression_info(
    &self,
    _acc: Self::BinOptAccumulator,
    bin: &mut BinCompressionInfo<U>,
  ) {
    bin.offset_bits = bits::bits_to_encode_offset(bin.upper - bin.lower);
  }

  #[inline]
  fn calc_offset(u: U, bin: &BinCompressionInfo<U>) -> U {
    u - bin.lower
  }

  #[inline]
  fn unchecked_decompress_unsigned(bin: &BinDecompressionInfo<U>, reader: &mut BitReader) -> U {
    bin.lower + reader.unchecked_read_uint::<U>(bin.offset_bits)
  }

  #[inline]
  fn decompress_unsigned(bin: &BinDecompressionInfo<U>, reader: &mut BitReader) -> PcoResult<U> {
    let offset = reader.read_uint::<U>(bin.offset_bits)?;
    Ok(bin.lower + offset)
  }
}
