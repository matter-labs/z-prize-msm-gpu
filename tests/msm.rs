use ark_bls12_377::G1Affine;
use ark_ec::msm::VariableBaseMSM;
use ark_ec::ProjectiveCurve;
use ark_ff::BigInteger256;
use std::str::FromStr;
use z_prize_msm_gpu::*;

#[test]
fn msm_correctness() {
    let test_npow = std::env::var("TEST_NPOW").unwrap_or("26".to_string());
    let npoints_npow = i32::from_str(&test_npow).unwrap();
    let batches = 4;
    let (points, scalars) =
        util::generate_points_scalars::<G1Affine>(1usize << npoints_npow, batches);
    let mut context = create_msm_context(points.as_slice());
    let scalars = unsafe { std::mem::transmute::<&[_], &[BigInteger256]>(scalars.as_slice()) };
    let msm_results = execute_batch_msm(&mut context, scalars);

    for b in 0..batches {
        let start = b * points.len();
        let end = (b + 1) * points.len();

        let arkworks_result =
            VariableBaseMSM::multi_scalar_mul(points.as_slice(), &scalars[start..end])
                .into_affine();

        assert_eq!(msm_results[b].into_affine(), arkworks_result);
    }
}
