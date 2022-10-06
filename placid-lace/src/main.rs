use euphony::prelude::*;

static BD: Buffer =
    Buffer::new("https://github.com/tidalcycles/sounds-tr808-fischer/raw/main/bd8/BD0010.WAV");
static SD: Buffer =
    Buffer::new("https://github.com/tidalcycles/sounds-tr808-fischer/raw/main/sd8/SD0010.WAV");
static HH: Buffer =
    Buffer::new("https://github.com/tidalcycles/sounds-tr808-fischer/raw/main/ch8/CH.WAV");

const PHRASE: Beat = Beat(9, 1);

#[euphony::main]
async fn main() {
    set_tempo(Tempo(150, 1));

    async {
        let b = rand::rhythm(PHRASE, [Beat(1, 1), Beat(2, 1), Beat(1, 2)]);
        for _ in 0..4 {
            for b in b.iter() {
                BD.play().spawn_primary();
                b.delay().await;
            }
        }
    }
    .seed(20)
    .group("bass-drum")
    .spawn_primary();

    async {
        let b = rand::rhythm(PHRASE, [Beat(1, 1), Beat(2, 1), Beat(3, 1), Beat(4, 1)]);
        for _ in 0..4 {
            for b in b.iter() {
                b.delay().await;
                SD.play().spawn_primary();
            }
        }
    }
    .seed(16)
    .group("snare")
    .spawn_primary();

    async {
        let b = rand::rhythm(PHRASE / 4, [Beat(1, 2), Beat(1, 4)]);
        let b2 = rand::rhythm(PHRASE / 4, [Beat(1, 2), Beat(1, 4)]);
        let b3 = rand::rhythm(PHRASE / 4, [Beat(1, 2), Beat(1, 4)]);
        let b4 = rand::rhythm(PHRASE / 4, [Beat(1, 2), Beat(1, 4)]);
        for _ in 0..4 {
            for b in b.iter().chain(&b2).chain(&b3).chain(&b4) {
                HH.play().spawn_primary();
                b.delay().await;
            }
        }
    }
    .seed(11)
    .group("hi-hat")
    .spawn_primary();

    async {
        let b = rand::rhythm(PHRASE / 2, [Beat(2, 1), Beat(1, 1), Beat(1, 2)]);
        let b2 = rand::rhythm(PHRASE / 2, [Beat(2, 1), Beat(1, 1), Beat(1, 2)]);
        let n: Vec<_> = b
            .iter()
            .map(|_| {
                rand::one_of(&[
                    Interval(0, 1),
                    Interval(1, 7),
                    Interval(2, 7),
                    Interval(3, 7),
                    Interval(4, 7),
                ])
            })
            .collect();
        let n2: Vec<_> = b2
            .iter()
            .map(|_| {
                rand::one_of(&[
                    Interval(-2, 7),
                    Interval(-1, 7),
                    Interval(0, 1),
                    Interval(2, 7),
                    Interval(3, 7),
                    Interval(4, 7),
                ])
            })
            .collect();
        let intervals = [
            Interval(rand::gen_range(-4..4), 7),
            Interval(rand::gen_range(-4..4), 7),
        ];
        let m = [
            mode::western::AEOLIAN,
            mode::western::DORIAN,
            mode::western::LYDIAN,
            mode::western::LOCRIAN,
            mode::western::PHRYGIAN,
        ];
        let modes = [*rand::one_of(&m), *rand::one_of(&m)];
        for (interval, mode) in intervals.iter().zip(modes.iter()).cycle().take(4) {
            let interval = *interval + Interval(2, 1);
            for (b, note) in b.iter().zip(n.iter()).chain(b2.iter().zip(n2.iter())) {
                let note = *mode * (**note - interval);
                let note = note
                    * tuning::Tuning {
                        base: BaseFrequency(440, 1),
                        system: &tuning::ET12,
                    };
                bass(note).group("bass").spawn_primary();
                b.delay().await;
            }
        }
    }
    .seed(41)
    .group("bass")
    .spawn_primary();
}

async fn bass(frequency: Frequency) {
    let v = env::linear().with_duration(0.01).with_target(0.5);

    let cutoff = osc::sine().with_frequency(frequency * 24.0);

    let freq = frequency;

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
