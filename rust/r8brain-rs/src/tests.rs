use super::*;

#[test]
fn test_resampler_basic() {
    let input = [
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3,
        0.2, 0.1, 0.0,
    ];

    // 96k to 48k ..
    let mut resampler = Resampler::new(2.0, 1.0, 32, 2.0, PrecisionProfile::Bits24);
    let mut storage = [0.0f64; 512];

    let mut empty_outputs = 0;
    for i in 0..0xff {
        let out_len = resampler.process(input.as_slice(), &mut storage);
        if out_len == 0 {
            empty_outputs += 1;
            continue;
        }
        println!("{i:03}{result:?}", result = &storage[..out_len], i = i);
    }

    println!(
        "it took {null_outputs} calls to get a non-empty slice back",
        null_outputs = empty_outputs
    );
}

#[test]
fn test_resampler_flush() {
    let input = [
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3,
        0.2, 0.1, 0.0,
    ];
    let mut resampler = Resampler::new(2.0, 1.0, 128, 2.0, PrecisionProfile::Bits24);
    let mut storage = [0.0f64; 4096];
    for _ in 0..0xff {
        resampler.process(input.as_slice(), &mut storage);
    }

    let flush_len = resampler.flush(&mut storage);
    println!("{:?}", &storage[..flush_len]);
    println!("flush size: {flush_len}", flush_len = flush_len);
}

#[test]
fn test_resampler_input_len() {
    let input = [
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3,
        0.2, 0.1, 0.0,
    ];
    let mut resampler = Resampler::new(2.0, 1.0, 128, 2.0, PrecisionProfile::Bits24);
    let mut storage = [0.0f64; 4096];

    assert_eq!(resampler.input_len_for_output_pos(0), 3388);

    let mut out_pos = 0;
    let mut inp_pos = 0;

    for _ in 0..0xff {
        inp_pos += input.len();
        out_pos += resampler.process(input.as_slice(), &mut storage);
    }

    assert_eq!(
        resampler.input_len_for_output_pos(out_pos + 0xff) - inp_pos,
        2 * 0xff + 1
    );
}

#[cfg(bench)]
mod benchmarks {

    #[bench]
    fn resample_96k_to_48k_16bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler = Resampler::new(2.0, 1.0, base_len * 10, 2.0, PrecisionProfile::Bits16);
        let mut storage = [0.0f64; 512];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }

    #[bench]
    fn resample_48k_to_96k_16bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler = Resampler::new(1.0, 2.0, base_len * 10, 2.0, PrecisionProfile::Bits16);
        let mut storage = [0.0f64; 512];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }

    #[bench]
    fn resample_48k_to_96k_24bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler = Resampler::new(1.0, 2.0, base_len * 10, 2.0, PrecisionProfile::Bits24);
        let mut storage = [0.0f64; 512];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }

    #[bench]
    fn resample_48k_to_192k_24bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler = Resampler::new(1.0, 4.0, base_len * 10, 2.0, PrecisionProfile::Bits24);
        let mut storage = [0.0f64; 1024];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }

    #[bench]
    fn resample_44dot1k_to_192k_24bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler =
            Resampler::new(44.1, 192.0, base_len * 10, 2.0, PrecisionProfile::Bits24);
        let mut storage = [0.0f64; 1024];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }

    #[bench]
    fn resample_192k_to_44dot1k_24bit(b: &mut Bencher) {
        let input = [
            0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4,
            0.3, 0.2, 0.1, 0.0,
        ];
        let base_len = input.len();
        let big_vec = input
            .into_iter()
            .cycle()
            .take(base_len * 10)
            .collect::<Vec<_>>();

        let mut resampler =
            Resampler::new(192.0, 44.1, base_len * 10, 2.0, PrecisionProfile::Bits24);
        let mut storage = [0.0f64; 1024];

        b.bytes = (base_len * std::mem::size_of::<f64>()) as u64;
        b.iter(|| {
            resampler.process(big_vec.as_slice(), &mut storage);
        });
    }
}
