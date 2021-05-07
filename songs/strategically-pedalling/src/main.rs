euphony::prelude!();
use euphony::pitch::mode::western::*;

const PHRASE_LEN: Beat = Beat((7 + 5) * 4, 4);

#[euphony::main]
async fn main() {
    section(PHRASE_LEN * 2).with(organ()).await;
    section(PHRASE_LEN * 4)
        .with(bd())
        .with(sd())
        .with(hh())
        .with(crash())
        .with(bass())
        .with(organ())
        .with(tinkle())
        .await;
    section(PHRASE_LEN / 2)
        .with(bd())
        .with(sd())
        .with(hh())
        .with(tinkle())
        .await;
}

async fn organ() {
    let t = track("organ");

    let oct = Interval(5, 1);

    for mode in [DORIAN, MIXOLYDIAN, MINOR].iter().copied().cycle() {
        for int in [0, 2, 2, 6].iter().copied() {
            for beat in [Beat(5, 4), Beat(1, 2)].iter().copied() {
                let mut p = vec![];
                for offset in [0, 2, 4].iter().copied() {
                    let int = Interval(int + offset, 7) + oct;
                    let freq = to_freq(int, mode);
                    p.push(t.send(synths::organ().freq(freq).amp(0.2)));
                }
                beat.delay().await;
            }
        }

        let oct = oct + Interval(3, 12);

        for int in [0, 1, 2, -2].iter().copied() {
            for beat in [Beat(3, 4), Beat(1, 2)].iter().copied() {
                let mut p = vec![];
                for offset in [0, 2, 4, 6].iter().copied() {
                    let int = Interval(int + offset, 7) + oct;
                    let freq = to_freq(int, mode);
                    p.push(t.send(synths::organ().freq(freq).amp(0.2)));
                }
                beat.delay().await;
            }
        }
    }
}

async fn bass() {
    let t = track("bass");

    for mode in [DORIAN, MIXOLYDIAN, MINOR].iter().copied().cycle() {
        for tonic in [Interval(3, 1)].iter().copied() {
            for int in [0, 2, 2, 6].iter().copied() {
                for beat in [Beat(1, 4), Beat(1, 1), Beat(1, 2)].iter().copied() {
                    let int = Interval(int, 7) + tonic;
                    let freq = to_freq(int, mode);
                    let n = t.send(synths::bass().freq(freq).amp(0.35));
                    beat.delay().await;
                    drop(n);
                }
            }
        }

        for tonic in [Interval(3, 1) + Interval(3, 12)].iter().copied() {
            for int in [0, 1, 2, -2].iter().copied() {
                for beat in [Beat(1, 2), Beat(1, 2), Beat(1, 4)].iter().copied() {
                    let int = Interval(int, 7) + tonic;
                    let freq = to_freq(int, mode);
                    let n = t.send(synths::bass().freq(freq).amp(0.35));
                    beat.delay().await;
                    drop(n);
                }
            }
        }
    }
}

async fn tinkle() {
    let t = track("tinkle");
    let oct = Interval(8, 1);

    let max = 16i32;
    let pan = 0i32..max;
    let mut pan = pan
        .clone()
        .chain(pan.rev())
        .map(|v| v as f32 / max as f32 * 2.0 - 0.5)
        .cycle();

    for mode in [DORIAN, MIXOLYDIAN, MINOR].iter().copied().cycle() {
        for int in [0, 4, 0, 2].iter().copied() {
            for _ in 0u8..14 {
                let int = Interval(int, 7) + oct;
                let freq = to_freq(int, mode);
                let n = t.send(
                    synths::tinkle()
                        .freq(freq)
                        .amp(0.02)
                        .pan(pan.next().unwrap()),
                );
                Beat(1, 8).delay().await;
                drop(n);
            }
        }

        let oct = oct + Interval(3, 12);

        for int in [0, 3, 4, -4].iter().copied() {
            for _ in 0u8..10 {
                let int = Interval(int, 7) + oct;
                let freq = to_freq(int, mode);
                let n = t.send(
                    synths::tinkle()
                        .freq(freq)
                        .amp(0.02)
                        .pan(pan.next().unwrap()),
                );
                Beat(1, 8).delay().await;
                drop(n)
            }
        }
    }
}

async fn bd() {
    let t = track("drums");
    for beat in [Beat(1, 4), Beat(2, 1), Beat(3, 2), Beat(1, 1)]
        .iter()
        .cycle()
        .copied()
    {
        t.send(assets::bd);
        beat.delay().await;
    }
}

async fn sd() {
    let t = track("drums");
    Beat(1, 1).delay().await;
    for beat in [Beat(2, 1), Beat(3, 2), Beat(1, 1)].iter().cycle().copied() {
        t.send(assets::sd[2]);
        beat.delay().await;
    }
}

async fn hh() {
    let t = track("drums");

    for beat in [Beat(1, 1), Beat(1, 4), Beat(1, 4), Beat(1, 2)]
        .iter()
        .cycle()
        .copied()
    {
        t.send(assets::hh);
        beat.delay().await;
    }
}

async fn crash() {
    let t = track("drums");
    loop {
        t.send(assets::cy);
        (PHRASE_LEN * 2i64).delay().await;
    }
}

fn to_freq(interval: Interval, mode: euphony::pitch::mode::Mode) -> f32 {
    let note: Interval = (mode * interval) * 12;
    let note = note.whole();
    let note = note as i32;
    let note = note as f32;
    2f32.powf((note - 69f32) / 12f32) * 440.0
}
