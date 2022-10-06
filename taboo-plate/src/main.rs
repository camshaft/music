use euphony::prelude::{samples::dirt, *};
use western::*;

const LEN: usize = 4;

static MODE: Cell<Mode> = Cell::new(MAJOR);

async fn modes() {
    for mode in [MINOR, MAJOR, DORIAN, MIXOLYDIAN].iter().cycle().take(64) {
        MODE.set(*mode);
        Beat(8, 1).delay().await;
    }
}

static TONIC: Cell<Interval> = Cell::new(Interval(0, 1));

fn to_frequency(i: Interval) -> Frequency {
    (i + *TONIC) * *MODE * ET12
}

async fn bass_voice(freq: Frequency, depth: f64) {
    let v = env::linear().with_duration(0.001).with_target(0.9);

    let freq = osc::sine().with_frequency(freq * 0.25).mul_add(depth, freq);

    let t = osc::wave()
        .with_buffer(&samples::akwf::eorgan[3])
        .with_frequency(freq);

    let t = t * &v;
    let t = t.sink();

    Beat(1, 32).delay().await;
    v.set_target(0.5);

    Beat(1, 2).delay().await;

    v.set_duration(0.01);
    v.set_target(0.0);

    Beat(1, 32).delay().await;

    t.fin();
}

async fn bass() {
    for _ in 0..LEN {
        for v in 0..8 {
            for i in 0..3 {
                bass_voice(to_frequency(Interval(i * 2, 7) - 2), v as _)
                    .group("bass")
                    .spawn_primary();
                Beat(1, 3).delay().await;
            }
        }
    }
}

async fn lead_voice(freq: Frequency, amp: f64, idx: usize) {
    let v = env::linear().with_duration(0.1).with_target(amp);

    let t = osc::wave()
        .with_buffer(&samples::akwf::sinharm[idx])
        .with_frequency(freq);

    let t = t * &v;
    let t = t.sink();

    Beat(1, 32).delay().await;
    v.set_target(amp * 0.9);

    Beat(1, 2).delay().await;

    v.set_duration(0.01);
    v.set_target(0.0);

    Beat(1, 32).delay().await;

    t.fin();
}

async fn lead() {
    let b1 = rand::rhythm(Beat(8, 1), Beat::vec([1, 3]));
    let i = b1.each(|_| rand::gen_range(-4..=4));
    let v = b1.each(|_| *rand::one_of(&[0.9, 0.75, 0.5, 0.25]));
    let d = b1.each(|_| rand::gen_range(0..usize::MAX));

    for _ in 0..LEN {
        for (beat, i, v, d) in (&b1, &i, &v, &d).zip() {
            lead_voice(to_frequency(Interval(*i, 7)), *v, *d)
                .group("lead")
                .spawn_primary();
            beat.delay().await;
        }
    }
}

async fn kick() {
    for _ in 0..LEN {
        for _ in 0..4 {
            dirt::bd.play().spawn_primary();
            Beat(2, 1).delay().await;
        }
    }
}

async fn snare() {
    Beat(1, 1).delay().await;

    for _ in 0..LEN {
        for _ in 0..4 {
            dirt::sd.play().spawn_primary();
            Beat(2, 1).delay().await;
        }
    }
}

async fn hihat() {
    let b1 = rand::rhythm(Beat(8, 1), Beat::vec([3, 6]));
    let i = b1.each(|_| rand::gen_range(0usize..20));
    let v = b1.each(|_| *rand::one_of(&[0.5, 0.25]));

    for _ in 0..LEN {
        for (beat, i, v) in (&b1, &i, &v).zip() {
            let p = dirt::uxay[*i].play();
            let delay = p.delay();
            let sink = p.mul(*v).sink();
            async move {
                delay.await;
                sink.fin()
            }
            .spawn_primary();
            beat.delay().await;
        }
    }
}

async fn speek() {
    let b1 = rand::rhythm(Beat(8, 1), Beat::vec([3, 6, 9]));
    let i = b1.each(|_| rand::gen_range(0usize..20));
    let v = b1.each(|_| *rand::one_of(&[1.0, 0.75, 0.5, 0.25]));

    for _ in 0..LEN {
        for (beat, i, v) in (&b1, &i, &v).zip() {
            let p = dirt::speakspell[*i].play();
            let delay = p.delay();
            let sink = p.mul(*v).sink();
            async move {
                delay.await;
                sink.fin()
            }
            .spawn_primary();
            beat.delay().await;
        }
    }
}

#[euphony::main]
async fn main() {
    set_tempo(Tempo(100, 1));

    modes().seed(4).spawn();
    bass().group("bass").seed(5).spawn_primary();
    hihat().group("hihat").seed(405).spawn_primary();

    let d = Beat(8 * LEN as u64, 1);

    async move {
        let b = 2;

        (d - Beat(b, 1)).delay().await;

        let count = 6;

        for _ in 0..count {
            let i = rand::gen();
            dirt::drum[i].play().spawn();
            Beat(b, count).delay().await;
        }
    }
    .seed(9)
    .spawn_primary();

    d.delay().await;

    modes().seed(4).spawn();
    lead().group("lead").seed(24).spawn_primary();
    bass().group("bass").seed(5).spawn_primary();
    kick().group("kick").seed(2).spawn_primary();
    snare().group("snare").seed(2).spawn_primary();
    hihat().group("hihat").seed(405).spawn_primary();
    speek().group("speek").seed(15).spawn_primary();
}
