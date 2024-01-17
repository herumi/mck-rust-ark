use ark_bls12_381::Fr;
use ark_bls12_381::*;
use ark_ec::*;
use ark_ff::*;
use ark_std::*;
use mcl::*;
use mcl_rust as mcl;

fn to_ptr_fp(x: &ark_bls12_381::Fq) -> *const mcl::Fp {
    let p: *const _ = x;
    let yp = p as *const mcl::Fp;
    return yp;
}

fn to_ptr_fr(x: &ark_bls12_381::Fr) -> *const mcl::Fr {
    let p: *const _ = x;
    let yp = p as *const mcl::Fr;
    return yp;
}

fn to_g1(p: &ark_bls12_381::G1Affine) -> mcl::G1 {
    unsafe {
        let mut ret = mcl::G1::uninit();
        let px = to_ptr_fp(&p.x);
        let py = to_ptr_fp(&p.y);
        ret.x = (*px).clone();
        ret.y = (*py).clone();
        ret.z.set_int(1);
        return ret;
    }
}


fn main() {
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
        let yp = to_ptr_fp(&a.x);
        unsafe {
            println!("yy={}\n", (*yp).get_str(10));
        }
        let g = to_g1(&a);
        println!("g={}\n", g.get_str(10));
    }
    {
        let a = G1Affine::rand(&mut rng);
        let b = G1Affine::rand(&mut rng);

        let s1 = Fr::rand(&mut rng);
        let s2 = Fr::rand(&mut rng);
        println!("s1={}\n", s1);
        unsafe {
            println!("mcl s1={}\n", (*to_ptr_fr(&s1)).get_str(10));
        }
        println!("s2={}\n", s2);
        let r = G1Projective::msm(&[a, b], &[s1, s2]).unwrap();
        println!("rr={}\n", to_g1(&r.into_affine()).get_str(10));

        let mut xs: Vec<_> = Vec::new();
        let mut ys: Vec<_> = Vec::new();
        xs.resize_with(2, Default::default);
        ys.resize_with(2, Default::default);
        unsafe {
            xs[0] = to_g1(&a);
            xs[1] = to_g1(&b);
            ys[0] = (*to_ptr_fr(&s1)).clone();
            ys[1] = (*to_ptr_fr(&s2)).clone();
        }
        let mut g1 = unsafe { <mcl::G1>::uninit() };
        mcl::G1::mul_vec(&mut g1, &xs, &ys);
        println!("g1={}\n", g1.get_str(10));
    }

	{
		let x = ark_ff::biginteger::BigInteger256::new([1,2,3,0xffffffffffffffff]);
		println!("x={}\n", x);
		let mut y:mcl::Fr = mcl::Fr::zero();
		y.set_little_endian_mod(&x.to_bytes_le());
		println!("m={}\n", y.get_str(10));
		println!("m={}\n", y.get_str(16));
	}
    /*
        let num_points = 2;
        let mut rng = ark_std::test_rng();
        let mut points: Vec<_> = Vec::new();
        let mut scalars: Vec<_> = Vec::new();
        for _ in 0..num_points {
            points.push(G1Affine::from(<G1Affine as AffineCurve>::Projective::rand(
                &mut rng,
            )));
            scalars.push(<G1Affine as AffineCurve>::ScalarField::rand(&mut rng).into_repr());
        }

        verify_correctness(&points, &scalars, 14);
    */
}
