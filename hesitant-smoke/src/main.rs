use euphony::prelude::*;
use samples::dirt;
use western::*;

/// Sets the duration of the composition
const LENGTH: Beat = Beat(16, 1);

static MODE: Cell<Mode> = Cell::new(MINOR);

#[euphony::main]
async fn main() {
    macro_rules! perc {
        ($durations:expr, $samples:expr) => {
            async {
                let samples = $samples;
                let beats = rand::rhythm(LENGTH, $durations);
                let sample = beats.each(|_| *samples.pick());
                loop {
                    let mut sample = (&beats).delays().with(&sample);
                    while let Some(sample) = sample.next().await {
                        sample.play().spawn();
                    }
                }
            }
        };
    }

    macro_rules! synth {
        ($durations:expr, $octave:expr) => {
            synth!($durations, $octave, Beat(1, 4), Beat(1, 8))
        };
        ($durations:expr, $octave:expr, $sustain:expr, $decay:expr) => {
            async {
                let beats = rand::rhythm(LENGTH, $durations);
                let intervals = beats.each(|_| Interval(*[0, 2, 3, 4].pick(), 7));
                loop {
                    let mut events = (&beats).delays().with(&intervals);
                    while let Some(interval) = events.next().await {
                        synth(*interval + $octave, $sustain, $decay).spawn();
                    }
                }
            }
        };
    }

    async fn lead() {
        synth!(Beat::vec([1, 2]), -1)
            .seed(12356)
            .group("lead")
            .await;
    }

    async fn harmony() {
        synth!(Beat::vec([1, 2, 4]), -2)
            .seed(123)
            .group("harmony")
            .await;
    }

    async fn high() {
        synth!([Beat(1, 1), Beat(2, 1)], 0, Beat(3, 4), Beat(1, 4))
            .seed(1989)
            .group("high")
            .await;
    }

    async fn high2() {
        synth!(
            [Beat(1, 1), Beat(2, 1), Beat(1, 2), Beat(1, 4)],
            0,
            Beat(1, 8),
            Beat(1, 16)
        )
        .seed(140)
        .group("high-2")
        .await;
    }

    async fn bass() {
        async {
            let beats = rand::rhythm(LENGTH, [Beat(1, 1), Beat(2, 1)]);
            let intervals = beats.each(|_| Interval(*[0, 2, 3, 4].pick(), 7));
            loop {
                let mut events = (&beats).delays().with(&intervals);
                while let Some(interval) = events.next().await {
                    bass_voice(*interval - 2).spawn();
                }
            }
        }
        .seed(5)
        .group("bass")
        .await;
    }

    async fn bd() {
        perc!([Beat(2, 1), Beat(1, 1)], &dirt::bd)
            .group("bd")
            .seed(123)
            .await;
    }

    async fn sd() {
        async {
            delay!(1);
            perc!([Beat(2, 1), Beat(1, 1)], [&dirt::sd]).await;
        }
        .group("sd")
        .seed(1234)
        .await;
    }

    async fn hh() {
        perc!(Beat::vec([2, 4]), &dirt::uxay)
            .group("hh")
            .seed(1234)
            .await;
    }

    section(LENGTH)
        .with(bass())
        .with(harmony())
        .with(hh())
        .with(bd())
        .await;
    section(LENGTH)
        .with(bass())
        .with(lead())
        .with(harmony())
        .with(hh())
        .with(bd())
        .await;

    MODE.set(MAJOR);
    for _ in 0..2 {
        section(LENGTH)
            .with(bass())
            .with(lead())
            .with(high())
            .with(high2())
            .with(harmony())
            .with(hh())
            .with(bd())
            .with(sd())
            .await;
        MODE.set(MINOR);
    }

    MODE.set(MAJOR);
    for _ in 0..2 {
        section(LENGTH)
            .with(bass())
            .with(lead())
            .with(high())
            .with(high2())
            .with(hh())
            .with(bd())
            .await;
    }

    MODE.set(MINOR);
    for _ in 0..2 {
        section(LENGTH)
            .with(bass())
            .with(lead())
            .with(high())
            .with(high2())
            .with(harmony())
            .with(hh())
            .with(bd())
            .with(sd())
            .await;
        MODE.set(MAJOR);
    }
}

async fn bass_voice(interval: Interval) {
    let freq = interval * *MODE * ET12;

    let osc = osc::sine().with_frequency(freq);

    let attack = Beat(1, 64);
    let sustain = Beat(1, 4);
    let decay = Beat(1, 16);

    let env = env::linear().with_duration(attack).with_target(0.5);

    let sink = (osc * &env).sink();

    delay!(sustain);

    env.set_duration(decay);
    env.set_target(0.0);

    delay!(decay);

    sink.fin();
}

async fn synth(interval: Interval, sustain: Beat, decay: Beat) {
    let freq = interval * *MODE * ET12;

    let attack = Beat(1, 64);

    let detune = freq.0 / 256.0;

    macro_rules! osc {
        () => {
            osc::sawtooth().with_frequency(freq.0 + (-detune..detune).pick()) * 0.2
        };
    }

    let oscs = osc!() + osc!() + osc!() + osc!() + osc!();

    let cutoff = env::linear()
        .with_duration(attack + sustain + decay)
        .with_value(15000.0)
        .with_target(5000.0);
    let oscs = oscs.moog().with_cutoff(cutoff);

    let env = env::linear().with_duration(attack).with_target(0.5);

    let sink = (oscs * &env).sink();

    delay!(sustain);

    env.set_duration(decay);
    env.set_target(0.0);

    delay!(decay);

    sink.fin();
}
