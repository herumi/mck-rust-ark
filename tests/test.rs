use ark_bls12_381::Fr;
use ark_bls12_381::*;
use ark_ec::*;
use ark_ff::*;
use ark_std::*;
use mcl::*;
use mcl_ark::*;
use mcl_rust as mcl;

#[test]
fn test_main() {
    mcl::init(CurveType::BLS12_381);
    let mut rng = ark_std::test_rng();
    let p: G1Affine = G1Projective::rand(&mut rng).into();
    assert_eq!(p.x.to_string(), to_fp(&p.x).get_str(10));
    assert_eq!(p.y.to_string(), to_fp(&p.y).get_str(10));

    unsafe {
        let mut mx = mcl::Fp::uninit();
        let ok = mx.deserialize(&p.x.into_bigint().to_bytes_le());
        assert!(ok);
        assert_eq!(p.x.to_string(), mx.get_str(10));
    }
    {
        let mp = to_g1(&p);
        assert_eq!(mp.x, to_fp(&p.x));
        assert_eq!(mp.y, to_fp(&p.y));
    }
    {
        let x1 = G1Affine::rand(&mut rng);
        let x2 = G1Affine::rand(&mut rng);

        let y1 = Fr::rand(&mut rng);
        let y2 = Fr::rand(&mut rng);
        let r1 = G1Projective::msm(&[x1, x2], &[y1, y2]).unwrap();

        let mut xs: Vec<_> = Vec::new();
        let mut ys: Vec<_> = Vec::new();
        xs.push(to_g1(&x1));
        xs.push(to_g1(&x2));
        ys.push(to_fr(&y1));
        ys.push(to_fr(&y2));
        let mut g1 = unsafe { <mcl::G1>::uninit() };
        mcl::G1::mul_vec(&mut g1, &xs, &ys);
        assert_eq!(g1, to_g1(&r1.into_affine()));
    }
    //	for i in 0..10 {
    //		let x = ark_ff::biginteger::BigInteger256::rand(&mut rng);
    //	}
}
