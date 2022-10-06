use euphony::prelude::*;
use samples::dirt;
use western::*;

static MODE: Cell<Mode> = Cell::new(MAJOR);

async fn modes() {
    let count = rand::gen_range(1usize..=8);

    let modes = count.each(|_| *rand::one_of(&[DORIAN, MAJOR]));
    let beats = count.each(|_| *rand::one_of(&[Beat(8, 1), Beat(4, 1), Beat(2, 1)]));

    for (mode, beat) in (&modes, &beats).zip().cycle() {
        MODE.set(*mode);
        beat.delay().await;
    }
}

static TONIC: Cell<Interval> = Cell::new(Interval(0, 1));

fn to_frequency(i: Interval) -> Frequency {
    (i + *TONIC) * *MODE * ET12
}

async fn bass_voice(freq: Frequency) {
    let v = env::linear().with_duration(0.01).with_target(0.5);

    let cutoff = osc::sine().with_frequency(freq * 24.0);

    let t = osc::triangle()
        .with_frequency(freq)
        .lowpass()
        .with_cutoff(4000.0 + 1000.0 * &cutoff);

    let sq = osc::nes::triangle()
        .with_frequency(freq * 2.0)
        .lowpass()
        .with_cutoff(500.0 + 200.0 * &cutoff)
        .highpass()
        .with_cutoff(50.0);

    let sq2 = osc::pulse()
        .with_frequency(freq * 0.98)
        .lowpass()
        .with_cutoff(1000.0)
        * 0.4;

    let t = t + sq + sq2;
    let t = t * &v;
    let t = t.sink();

    Beat(7, 8).delay().await;

    v.set_duration(0.01);
    v.set_target(0.0);

    Beat(1, 8).delay().await;

    t.fin();
}

async fn bass() {
    let b1 = rand::rhythm(Beat(8, 1), Beat::vec([1, 2]));
    let i1 = b1.each(|_| Interval(rand::gen_range(-4..=4), 7) - 2);
    let b2 = rand::rhythm(Beat(8, 1), Beat::vec([1, 2]));
    let i2 = b2.each(|_| Interval(rand::gen_range(-4..=4), 7) - 2);

    for _ in 0..2 {
        for (beat, interval) in (&b1, &i1).zip().chain((&b2, &i2).zip()) {
            bass_voice(to_frequency(*interval)).spawn_primary();
            beat.delay().await;
        }
    }
}

async fn kick() {
    let b1 = rand::rhythm(Beat(8, 1), [Beat(2, 1), Beat(1, 1)]);
    let b2 = rand::rhythm(Beat(8, 1), [Beat(2, 1), Beat(1, 1)]);

    for _ in 0..2 {
        for beat in b1.iter().chain(&b2).copied() {
            dirt::drum[2].play().spawn_primary();
            beat.delay().await;
        }
    }
}

async fn snare() {
    let b1 = rand::rhythm(Beat(8, 1), [Beat(2, 1), Beat(3, 1)]);
    let b2 = rand::rhythm(Beat(8, 1), [Beat(2, 1), Beat(3, 1)]);

    for _ in 0..2 {
        for beat in b1.iter().chain(&b2).copied() {
            beat.delay().await;
            dirt::drum[6].play().spawn_primary();
        }
    }
}

async fn hihat() {
    let b1 = rand::rhythm(Beat(8, 1), Beat::vec([1, 2]));
    let b2 = rand::rhythm(Beat(8, 1), Beat::vec([1, 2]));

    for _ in 0..2 {
        for beat in b1.iter().chain(&b2).copied() {
            dirt::drum[3].play().spawn_primary();
            beat.delay().await;
        }
    }
}

async fn lead(base: Interval) {
    let freq = env::linear().with_duration(0.01);
    let freq = &freq;
    let vol = env::linear().with_target(0.4);
    let vol = &vol;

    let lead = osc::pulse().with_frequency(freq).mul(vol);

    let cutoff = osc::triangle()
        .with_frequency(freq.div(2.0))
        .mul_add(2000.0, 3000.0);

    let lead = lead
        .lowpass()
        .with_cutoff(&cutoff)
        .lowpass()
        .with_cutoff(18000.0);

    let _lead = lead.sink();

    let b1 = rand::rhythm(Beat(4, 1), Beat::vec([1, 2, 4, 8]));
    let b1 = &b1;

    let i1 = b1.each(|_| Interval(*rand::one_of(&[0, 2, 4, 6, 7, 9]), 7));
    let i1 = &i1;

    let b2 = rand::rhythm(Beat(4, 1), Beat::vec([1, 2, 4, 8]));
    let b2 = &b2;

    let i2 = b2.each(|_| Interval(*rand::one_of(&[0, 2, 4, 6, 7, 9]), 7));
    let i2 = &i2;

    let melody = move |base| async move {
        for (beat, interval) in (b1, i1).zip().chain((b2, i2).zip()) {
            if interval.0 == 9 {
                vol.set_target(0.0);
            } else {
                vol.set_target(0.4);
                let target = to_frequency(*interval + base);
                freq.set_target(target);
            }
            beat.delay().await;
        }
    };

    let mut freq_d = (1..=3)
        .chain((2..3).rev())
        .map(|v| v as f64 / 100.0)
        .cycle();

    loop {
        melody(base).await;
        freq.set_duration(freq_d.next().unwrap());
    }
}

#[euphony::main]
async fn main() {
    section(Beat(8, 1) * 4)
        .with(modes().seed(4).spawn())
        .with(bass().group("bass").seed(5).spawn())
        .with(kick().group("kick").seed(2).spawn())
        .await;
    section(Beat(8, 1) * 4)
        .with(modes().seed(4).spawn())
        .with(lead(Interval(0, 1)).group("lead").seed(10000).spawn())
        .with(bass().group("bass").seed(5).spawn())
        .with(kick().group("kick").seed(2).spawn())
        .with(snare().group("snare").seed(4).spawn())
        .with(hihat().group("hihat").seed(4).spawn())
        .await;
    section(Beat(8, 1) * 4)
        .with(modes().seed(10).spawn())
        .with(lead(Interval(0, 1)).group("lead").seed(10000).spawn())
        .with(lead(Interval(2, 7)).group("lead").seed(10000).spawn())
        .with(bass().group("bass").seed(5).spawn())
        .with(kick().group("kick").seed(2).spawn())
        .with(snare().group("snare").seed(4).spawn())
        .with(hihat().group("hihat").seed(4).spawn())
        .await;
}
