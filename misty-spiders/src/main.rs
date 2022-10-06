use euphony::prelude::*;
use mode::western::*;
use tuning::{Tuning, ET12};

static TUNING: Tuning = Tuning {
    base: BaseFrequency(440, 1),
    system: &ET12,
};

static PROG: &[(mode::Mode, Interval)] = &[
    (MAJOR, Interval(0, 1)),
    (MINOR, Interval(2, 7)),
    (DORIAN, Interval(-2, 7)),
    (DORIAN, Interval(-1, 7)),
];

async fn bass(freq: Frequency) {
    let vol = env::linear().with_target(0.5).with_duration(0.001);
    let vol = &vol;

    let chorus = 0.5;

    let bass = osc::triangle().with_frequency(freq).mul(vol);
    let pulse = osc::sine().with_frequency(freq.0 + chorus).mul(vol * 0.2);
    let pulse2 = osc::sine().with_frequency(freq.0 - chorus).mul(vol * 0.1);

    let cutoff = osc::sine()
        .with_frequency(freq.0 / 2.0)
        .mul_add(1000.0, 2000.0);

    //let sink = bass;
    //let sink = bass + pulse;
    let sink = bass + pulse + pulse2;

    let sink = sink
        .lowpass()
        .with_cutoff(&cutoff)
        .lowpass()
        .with_cutoff(17_000.0);

    let sink = sink.sink();

    Beat(1, 2).delay().await;

    vol.set_target(0.0);
    vol.set_duration(0.1);

    Beat(1, 2).delay().await;

    sink.fin();
}

async fn bass_line() {
    let play = move |mode, base| async move {
        for beat in [Beat(1, 1), Beat(1, 1), Beat(3, 4)].iter() {
            let target = base * mode * TUNING;
            bass(target).group("bass").spawn_primary();
            beat.delay().await;
        }
    };

    for (mode, base) in PROG.iter().cycle().copied().take(32) {
        play(mode, base - 2).await;
    }
}

async fn lead() {
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

    let lead = lead.sink();

    let beats = [16u64, 16, 4, 8, 2, 8, 8, 4, 4, 16, 4, 16, 16, 4, 16, 4];
    let beats = &beats;

    let intervals = [0, 4, 4, 6, 9, 9, 3, 5, 5, 9, 8, 7, 6, 6, 1, 9];
    let intervals = &intervals;

    let melody = move |mode, base| async move {
        for (beat, interval) in beats.iter().zip(intervals.iter()) {
            if *interval == 9 {
                vol.set_target(0.0);
            } else {
                vol.set_target(0.4);
                let target = (Interval(*interval, 7) + base) * mode * TUNING;
                freq.set_target(target);
            }
            Beat(1, *beat).delay().await;
        }
    };

    let mut freq_d = (1..=3)
        .chain((2..3).rev())
        .map(|v| v as f64 / 100.0)
        .cycle();

    for (mode, base) in PROG.iter().cycle().copied().take(8) {
        melody(mode, base).await;
        freq.set_duration(freq_d.next().unwrap());
    }

    vol.set_target(0.0);

    lead.fin();
}

async fn lead_rand() {
    let freq = env::linear().with_duration(0.01);
    let freq = &freq;
    let vol = env::linear().with_target(0.4);
    let vol = &vol;

    let cutoff = osc::triangle()
        .with_frequency(freq.div(2.0))
        .mul_add(2000.0, 4000.0);

    let lead = osc::pulse()
        .with_frequency(freq)
        .mul(vol)
        .lowpass()
        .with_cutoff(&cutoff)
        .lowpass()
        .with_cutoff(18000.0)
        .sink();

    let beats: Vec<Vec<Beat>> = vec![
        rand::rhythm(Beat(3, 4) + 2u64, Beat::vec([1, 2, 4])),
        rand::rhythm(Beat(3, 4) + 2u64, Beat::vec([2, 4, 8])),
    ];
    let beats = &beats;

    let intervals = (0..16)
        .map(|_| rand::gen_range(0i64..=9))
        .collect::<Vec<_>>();
    let intervals = &intervals;

    let melody = move |mode, base, beats_idx: usize| async move {
        let b = &beats[beats_idx];
        let b = b.iter();
        for (beat, interval) in b.zip(intervals.iter()) {
            if *interval == 9 {
                vol.set_target(0.0);
            } else {
                vol.set_target(0.4);
                let target = (Interval(*interval, 7) + base) * mode * TUNING;
                freq.set_target(target);
            }
            beat.delay().await;
        }
    };

    let mut freq_d = (1..=5)
        .chain((2..5).rev())
        .map(|v| v as f64 / 100.0)
        .cycle();
    for (idx, (mode, base)) in PROG.iter().cycle().copied().take(8).enumerate() {
        melody(mode, base, idx / beats.len() % beats.len()).await;
        freq.set_duration(freq_d.next().unwrap());
    }

    vol.set_target(0.0);

    lead.fin();
}

#[euphony::main]
async fn main() {
    async {
        ((Beat(3, 4) + 2u64) * 8u64).delay().await;
        lead().await;
        let seed = 40;
        lead_rand().seed(seed).await;
        lead().await;
    }
    .group("lead")
    .spawn_primary();

    bass_line().spawn_primary();
}
