/*
Copyright 2022 Volker Schwaberow <volker@schwaberow.de>
Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including without
limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
Author(s): Volker Schwaberow
*/

use rand::rngs::OsRng;
use curve25519_dalek::{scalar::Scalar, constants};
use sha2::{Sha256, Digest};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    
    let mut rng = OsRng;
    let secret_key = Scalar::random(&mut rng);

    let g = &constants::RISTRETTO_BASEPOINT_POINT;
    let h = g * secret_key;

    let mut hasher = Sha256::new();
    hasher.update(h.compress().as_bytes());
    let vrf_out = hasher.finalize();

    let mut proof = Sha256::new();
    proof.update(vrf_out);

    let vrf_proof = proof.finalize();

    println!("VRF output: {:?}", vrf_out);
    println!("VRF proof: {:?}", vrf_proof);  
    Ok(())

}
