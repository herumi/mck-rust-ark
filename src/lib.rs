use ark_ff::BigInteger;
use mcl_rust as mcl;

pub fn to_ptr_fp(x: &ark_bls12_381::Fq) -> *const mcl::Fp {
    let p: *const _ = x;
    let yp = p as *const mcl::Fp;
    yp
}

pub fn to_fp(x: &ark_bls12_381::Fq) -> mcl::Fp {
    unsafe { (*to_ptr_fp(&x)).clone() }
}

pub fn to_ptr_fr(x: &ark_bls12_381::Fr) -> *const mcl::Fr {
    let p: *const _ = x;
    let yp = p as *const mcl::Fr;
    yp
}

pub fn to_fr(x: &ark_bls12_381::Fr) -> mcl::Fr {
    unsafe { (*to_ptr_fr(&x)).clone() }
}

pub fn to_g1(p: &ark_bls12_381::G1Affine) -> mcl::G1 {
    unsafe {
        let mut ret = mcl::G1::uninit();
        ret.x = to_fp(&p.x);
        ret.y = to_fp(&p.y);
        ret.z.set_int(1);
        ret
    }
}

pub fn mod_to_fr(x: &ark_ff::biginteger::BigInteger256) -> mcl::Fr {
    unsafe {
        let mut ret = mcl::Fr::uninit();
        ret.set_little_endian_mod(&x.to_bytes_le());
        ret
    }
}

pub fn to_frs(iv: &Vec<ark_bls12_381::Fr>) -> Vec<mcl::Fr> {
    let mut ov: Vec<_> = Vec::new();
    let n = iv.len();
    ov.reserve(n);
    for i in 0..n {
        ov.push(to_fr(&iv[i]));
    }
    return ov;
}

pub fn to_g1s(iv: &Vec<ark_bls12_381::G1Affine>) -> Vec<mcl::G1> {
    let mut ov: Vec<_> = Vec::new();
    let n = iv.len();
    ov.reserve(n);
    for i in 0..n {
        ov.push(to_g1(&iv[i]));
    }
    return ov;
}

pub fn mod_to_frs(iv: &Vec<ark_ff::biginteger::BigInteger256>) -> Vec<mcl::Fr> {
    let mut ov: Vec<_> = Vec::new();
    let n = iv.len();
    ov.reserve(n);
    for i in 0..n {
        ov.push(mod_to_fr(&iv[i]));
    }
    return ov;
}
