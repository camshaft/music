euphony::prelude!();

synthdef!(
    pub fn organ(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>) {
        let detune = [0.98, 0.99, 1.0, 1.01, 1.02];
        let freq = freq * detune;
        let signal = SinOsc::new().freq(freq).ar();
        let signal = Splay::new(signal).center(pan).ar() * amp;
        Out::new(out, signal).ar()
    }
);

synthdef!(
    pub fn bass(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>) {
        let detune = (1..20).map(|v| v as f32 * 0.05).collect::<Vec<_>>();
        let freq = freq * detune;
        let signal = SinOsc::new().freq(freq).ar();
        let signal = Splay::new(signal).center(pan).ar() * amp;
        Out::new(out, signal).ar()
    }
);
