euphony::prelude!();

#[euphony::main]
async fn main() {
    bass().spawn();
    drums().spawn();
    organ().await;
}

async fn organ() {
    let t = track("organ");
    for _ in 0u8..2 {
        for tonic in [440i32, 400, 500, 450].iter().copied() {
            for octave in (0i32..5).chain((1i32..4).rev()) {
                let octave = octave as f32;
                let tonic = tonic as f32 / 3.0;
                let freq = (octave / 4.0 + 1.0) * tonic;
                let n = t.send(synths::organ().freq(freq));
                Beat(1, 2).delay().await;
                drop(n);
            }
        }
    }
}

async fn bass() {
    let t = track("bass");

    for v in 1i32..=4 {
        for freq in [700, 400, 1400].iter().copied() {
            let n = t.send(synths::bass().freq(freq * v));
            Beat(1, 2).delay().await;
            drop(n);
        }
        (Beat(6, 1) + Beat(1, 2)).delay().await;
    }
}

async fn drums() {
    let t = track("drums");

    let i = [assets::cy, assets::hh, assets::sd, assets::bd];

    for _ in 1i32..=4 {
        for n in i.iter().cycle().copied().take(48) {
            t.send(n);
            Beat(1, 2).delay().await;
        }
    }
}
