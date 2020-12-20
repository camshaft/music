~b2t = { |beat| (beat / (1/4) * (60 / 160)) };

(
~rootR.stop;
~roots = Pseq([-3, 2, -3, 0], inf).asStream;
~rootR = Routine({
    loop({
        ~root = ~roots.next;
		~b2t.value(2).wait;
    })
}).play(quant: ~b2t.value(2));
)

(
~tuning = Tuning.at(\just);
~tunR.stop;
~tuns = Pseq([\just], inf).asStream;
~tunR = Routine({
    loop({
		~tuning = Tuning.at(~tuns.next.postln);
		~b2t.value(8).wait;
    })
}).play(quant: ~b2t.value(8));
)

(
~scale = Scale.at(\major);
~scaleR.stop;
~scls = Pseq([\major, \dorian], inf).asStream;
~scaleR = Routine({
    loop({
		~scale = Scale.newFromKey(~scls.next);
		~scale.tuning_(~tuning);
		~b2t.value(1).wait;
    })
}).play(quant: ~b2t.value(2));
)

(
  SynthDef(\arp, {
	|out = 0, freq = 440, amp = 0.8, pan = 0|
	var env, sig, det1, det2, f;

	env = Env.new([0, 1, 0], [0.01, 0.7]);
	env = EnvGen.ar(env, doneAction: 2);

	det1 = Vibrato.kr(freq, rate: 1, depth: 0.01, depthVariation: 0.1);
	det2 = Vibrato.kr(freq, rate: 2, depth: 0.01, depthVariation: 0.1);

	sig = [
		SinOsc.ar(det1),
		SinOsc.ar(det2),
	];
	sig = HPF.ar(in: sig, freq: 90.0, mul: 1.0, add: 0.0);
	sig = sig * env * amp;
	sig = Balance2.ar(sig[0], sig[1], pan);

	Out.ar(out, sig)
  }).add;
)

(
~arp.stop;
~arp = Pbind(
    \instrument, \arp,
	\scale, Pfunc({~scale}),
	\degree, Pseq([0, 1, 4, 2, 3, 6], inf),
	\dur, Pseq([1/6, 1/6, 1/6].collect(~b2t), inf),
	\pan, Pwhite(-0.5, 0.5, inf),
	\root, Pfunc({~root}),
	\amp, 0.05,
).play(quant: ~b2t.value(2));
)
~arp.stop;

(
  SynthDef(\hats, {
	|out=0, freq=50, t_trig=0, pan=0, amp=1.0, hpf = 1000|
	var env, sig;

	env = Env([0.0, 1.0, 0.0], [0.0, 0.01]);
	env = EnvGen.ar(env, doneAction: 2);

	sig = WhiteNoise.ar(1);
	sig = HPF.ar(sig, hpf);

	sig = sig * env * amp;
	sig = Pan2.ar(sig, pan);

	Out.ar(out, sig);
  }).add;
)

(
~hats.stop;
~hats = Pbind(
	\instrument, \hats,
	\dur, Prand([1/12].collect(~b2t), inf),
	\amp, Pseq(
		[
			Prand([0.05, 0.06, 0.07]),
	    ] ++ (Pexprand(0.0001, 0.05, 1) ! 5),
		inf
	),
	\pan, Pwhite(-0.5, 0.5, inf),
	\hpf, Prand([500, 1000, 5000, 12000], inf),
).play(quant: ~b2t.value(2));
)
~hats.stop

(
  SynthDef(\hats2, {
	|out=0, freq=50, t_trig=0, pan=0, amp=1.0, hpf = 1000, glitch = 0|
	var env, sig, glitchosc;

	env = Env([0.0, 1.0, 0.0], [0.0, 0.01]);
	env = EnvGen.ar(env, doneAction: 2);

	sig = WhiteNoise.ar(1) + glitch;
	sig = HPF.ar(sig, hpf);

	glitch = MouseX.kr(0, 4.0);
	glitchosc = Dust.ar(1, 0.5);
	glitch = CombN.ar(glitchosc, 0.01, 0.01, 3);

	sig = Mix.ar([sig, glitch]);

	sig = sig * env * amp;
	sig = Pan2.ar(sig, pan);

	Out.ar(out, sig);
  }).add;
)

(
~hats2.stop;
~hats2 = Pbind(
	\instrument, \hats2,
	\dur, Prand([1/6].collect(~b2t), inf),
	\amp, Pseq(
		[
			Prand([0.4, 0.5, 0.6]),
	    ] ++ (Pexprand(0.001, 0.4, 1) ! 5),
		inf
	),
	\pan, Pwhite(-0.5, 0.5, inf),
	\hpf, Prand([500, 1000, 5000, 12000], inf),
).play(quant: ~b2t.value(1));
)
~hats2.stop

(
  {
	|out=0, amp = 0.2, pan = 0, atk = 0.01, rel = 0.5|
	var env, sig, modfreq, carfreq;

	env = Env.perc(atk, rel);
	env = EnvGen.kr(env, doneAction: 2);

	//carfreq = Env([1.0, 0.0], [rel / 3], curve: \sqr);
	//carfreq = EnvGen.ar(carfreq);
	carfreq = 1;
	carfreq = carfreq * MouseY.kr(20, 10000);
	carfreq;

	modfreq = MouseX.kr(20, 10000);

	sig = SinOsc.ar(modfreq, mul: 200);
	sig = SinOsc.ar(carfreq + sig);
	sig = sig * env * amp;
	sig = Pan2.ar(sig, pan);

	Out.ar(out, sig);
  }.play;
)