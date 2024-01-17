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
    let a: G1Affine = G1Projective::rand(&mut rng).into();
    println!("a.x={}\n", a.x);
    println!("a.y={}\n", a.y);
    let x = a.x;
    let b = x.into_bigint().to_bytes_le();
    let mut xx = mcl::Fp::zero();
    let ok = xx.deserialize(&b);
    println!("ok={}\n", ok);
    println!("xx={}\n", xx.get_str(10));

    {
        println!("y={}\n", to_fp(&a.x).get_str(10));
        let g = to_g1(&a);
        println!("g={}\n", g.get_str(10));
    }
    {
        let a = G1Affine::rand(&mut rng);
        let b = G1Affine::rand(&mut rng);

        let s1 = Fr::rand(&mut rng);
        let s2 = Fr::rand(&mut rng);
        println!("s1={}\n", s1);
        println!("mcl s1={}\n", to_fr(&s1).get_str(10));
        println!("s2={}\n", s2);
        let r = G1Projective::msm(&[a, b], &[s1, s2]).unwrap();
        println!("rr={}\n", to_g1(&r.into_affine()).get_str(10));

        let mut xs: Vec<_> = Vec::new();
        let mut ys: Vec<_> = Vec::new();
        xs.push(to_g1(&a));
        xs.push(to_g1(&b));
        ys.push(to_fr(&s1));
        ys.push(to_fr(&s2));
        let mut g1 = unsafe { <mcl::G1>::uninit() };
        mcl::G1::mul_vec(&mut g1, &xs, &ys);
        println!("g1={}\n", g1.get_str(10));
    }

    {
        let x = ark_ff::biginteger::BigInteger256::new([1, 2, 3, 0xffffffffffffffff]);
        println!("x={}\n", x);
        let y = mod_to_fr(&x);
        println!("y={}\n", y.get_str(16));
    }
}
