mod errors {
    error_chain!{}
}

use self::errors::*;
use serde::{Deserialize, Serialize};

fn from_slice<T>(bytes: &[u8]) -> Result<T>
    where T: Deserialize
{
    unimplemented!()
}

fn to_vec<T>(data: T) -> Result<Vec<u8>>
    where T: Serialize
{
    unimplemented!()
}

mod ser;
mod de;
