use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use sp1_core::stark::StarkGenericConfig;
use sp1_core::utils::BabyBearPoseidon2;
use sp1_recursion_compiler::prelude::*;
use sp1_recursion_core::runtime::Runtime;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = b;
            b += a;
            a = temp;
        }
        a
    }
}

fn main() {
    let n_val = 12;
    let mut builder = AsmBuilder::<BabyBear>::new();
    let a: Felt<_> = builder.constant(BabyBear::zero());
    let b: Felt<_> = builder.constant(BabyBear::one());
    let n: Felt<_> = builder.constant(BabyBear::from_canonical_u32(n_val));

    let start: Felt<_> = builder.constant(BabyBear::zero());
    let end = n;

    builder.range(start, end).for_each(|_, builder| {
        let temp: Felt<_> = builder.uninit();
        builder.assign(temp, b);
        builder.assign(b, a + b);
        builder.assign(a, temp);
    });

    let expected_value = BabyBear::from_canonical_u32(fibonacci(n_val));
    builder.assert_eq(a, expected_value);

    let code = builder.code();
    println!("{}", code);

    let program = code.machine_code();

    type SC = BabyBearPoseidon2;
    type F = <SC as StarkGenericConfig>::Val;

    let mut runtime = Runtime::<F>::new(&program);
    runtime.run();
}
