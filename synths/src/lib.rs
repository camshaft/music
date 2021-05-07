use core::time::Duration;

euphony::prelude!();

synthdef!(
    #[drop(|mut synth| {
        synth.set().gate(0).send();
        synth.free_after(Duration::from_secs_f32(0.5));
    })]
    pub fn organ(out: f32<0>, freq: f32<440.0>, amp: f32<0.2>, pan: f32<0.0>, gate: f32<1>) {
        let env = Env::new()
            .levels([0., 1., 0.6, 0.])
            .times([0.01, 0.1, 0.5])
            .curve(Curve::Squared)
            .sustain(1)
            .xr();

        let detune = [0.99, 1.00, 1.01];
        let freq = freq * detune;

        let signal = SinOsc::new().freq(freq).ar();
        let signal = signal * EnvGen::new(env).gate(gate).done_action(2).kr();

        let signal = Splay::new(signal).center(pan).ar();
        let signal = signal * amp;

        Out::new(out, signal).ar()
    }
);

synthdef!(
    #[drop(|mut synth| {
        synth.set().gate(0).send();
        synth.free_after(Duration::from_secs_f32(0.5));
    })]
    pub fn bass(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>, gate: f32<1>) {
        let env = Env::new()
            .levels([0., 1., 0.6, 0.])
            .times([0.01, 0.1, 0.5])
            .curve(Curve::Squared)
            .sustain(1)
            .xr();

        let detune = [0.999, 1., 1.001];
        let freq = freq * detune;
        let signal = Pulse::new().freq(freq).ar();
        let signal = signal * EnvGen::new(env).gate(gate).done_action(2).kr();
        let signal = Splay::new(signal).center(pan).ar() * amp;

        let boost = SinOsc::new().freq(freq / 2).ar() * (amp / 2);
        let boost = boost * EnvGen::new(env).gate(gate).done_action(2).kr();

        let signal = signal + boost;

        Out::new(out, signal).ar()
    }
);

synthdef!(
    #[drop(|_| { })]
    pub fn tinkle(out: f32<0>, freq: f32<440.0>, amp: f32<0.5>, pan: f32<0.0>) {
        let env = Env::new()
            .levels([0., 1., 0.1, 0.])
            .times([0.001, 0.01, 0.1])
            .curve(Curve::Squared)
            .xr();
        let signal = SinOsc::new().freq(freq).ar();
        let signal = signal * EnvGen::new(env).done_action(2).kr();
        let signal = Pan2::new(signal).pos(pan).ar() * amp;
        Out::new(out, signal).ar()
    }
);
