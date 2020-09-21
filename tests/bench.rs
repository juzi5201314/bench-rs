use bench_rs::Bencher;
use bench_rs::bench;
use rand::Rng;

#[test]
fn test_bencher() {
    let data = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(1000).collect::<String>();

    let mut bencher = Bencher::new("test_bencher", 1000, data.len());
    bencher.iter(|| {
        let _ = rcnb_rs::encode(&data);
    });
    bencher.finish();
}

#[bench(name = "test_rcnb_encoding")]
fn test_rcnb(b: &mut Bencher) {
    let data = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(1000).collect::<String>();
    b.iter(|| {
        let _ = rcnb_rs::encode(&data);
    });
    b.bytes = data.len()
}

#[bench(count = 12345)]
fn test_base64(b: &mut Bencher) {
    let data = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(1000).collect::<String>();
    b.iter(|| {
        let _ = base64::encode(&data);
    });
    b.bytes = data.len()
}

#[bench(no_test)]
fn test_no_run(_: &mut Bencher) {
    println!("no #[test]");
}