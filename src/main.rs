use scale_value::{Primitive, Value};
use sp_core::H256;

/* Error: "invalid type: byte array, expected a (both 0x-prefixed or not) hex
 * string with length of 64"
 * (see primitive types fixed hash deserialization here: https://github.com/paritytech/parity-common/blob/aa5888340cd82f2dab2f2c4b5b65e1d0f4596482/primitive-types/impls/serde/src/serialize.rs#L236-L238)
 *
 * Appears that scale-value crate leaves the serialized version as byte array
 * but the sp_core primitive types expect a string. Can we serialize as string
 * instead?
 */

fn main() {
    // Simple encoding of H256 as U256 value fails
    let prim_encoded_h256 = Value::primitive(Primitive::U256(H256::zero().into()));
    println!("{:?}", &prim_encoded_h256);
    let _prim_deser_h256: H256 = scale_value::serde::from_value(prim_encoded_h256.clone()).unwrap(); // Fails, comment out line to see other failing case

    // Also fails when encoding as normal array of bytes
    let bytes_encoded_h256 = Value::from_bytes(H256::zero().as_bytes());
    println!("{:?}", &bytes_encoded_h256);
    let _bytes_deser_h256: H256 =
        scale_value::serde::from_value(prim_encoded_h256.clone()).unwrap(); // Fails
}
