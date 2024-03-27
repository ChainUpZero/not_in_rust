use bls12_381::Scalar;

mod small_ntt;
mod ntt_381;

fn main() -> Result<(), String> {
    for n in (3..=33).step_by(6) {
        let input: Vec<u64> = (0..=n).step_by(3).collect();
        println!("input vec  = {:?}", input);

        let output = small_ntt::ntt(&input)?;
        println!("after ntt  = {:?}", output);
        assert_ne!(input, output);

        let original = small_ntt::i_ntt(&output)?;
        println!("after inverse ntt = {:?}\n\n", original);
        assert_eq!(input, original);
    }

    for n in (3..=4).step_by(6) {
        let input: Vec<Scalar> = (0..=n).step_by(3).map(|x| Scalar::from(x)).collect();
        println!("input vec  = {:#?}", input);

        let output = ntt_381::ntt_381(&input);
        println!("after ntt  = {:#?}", output);
        assert_ne!(input, output);

        let original = ntt_381::i_ntt_381(&output);
        println!("after inverse ntt = {:#?}\n\n", original);
        // assert_eq!(input, original);
    }
    println!("{:#?} -> {:#?}", Scalar::from(1),<Scalar as Into<[u64;4]>>::into(Scalar::from(1)));
    Ok(())
}


