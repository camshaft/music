~b2t = { |beat| (beat / (1/4) * (60 / 120)) };

~quant = ~b2t.value(16);

(
~rootR.stop;
~roots = Pseq([-5, -4, -5, -3].collect({|r| r - 1}), inf).asStream;
~rootR = Routine({
    loop({
        ~root = ~roots.next.postln;
		~b2t.value(4).wait;
    })
}).play(quant: ~quant);

~tuning = Tuning.at(\just);
~tunR.stop;
~tuns = Pseq([\just], inf).asStream;
~tunR = Routine({
    loop({
		~tuning = Tuning.at(~tuns.next.postln);
		~b2t.value(8).wait;
    })
}).play(quant:~quant);

~scale = Scale.at(\major);
~scaleR.stop;
~scls = Pseq([
	Pseq([
		\major, \dorian, \phrygian, \mixolydian,
		\mixolydian, \mixolydian, \dorian, \phrygian,
	], 2),
], inf).asStream;
~scaleR = Routine({
    loop({
		~scale = Scale.newFromKey(~scls.next.postln);
		~scale.tuning_(~tuning);
		~b2t.value(1).wait;
    })
}).play(quant: ~quant);
)

(
  SynthDef(\arp, {
	|out = 0, freq = 440, amp = 0.8, pan = 0, attack = 0.005, release = 0.5|
	var env, sig, det1, det2, f;

	env = Env.new([0, 1, 0], [attack, release]);
	env = EnvGen.ar(env, doneAction: 2);

	sig = 2.collect({|r|
		var det = Vibrato.kr(freq, rate: r, depth: r * 0.01, depthVariation: 0.1);
		SinOsc.ar(det)
	});

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
	\degree, Pseq([0, 2], inf),
	\dur, Pseq([1/8, 1/4, 1/8, 1/4, 1/8].collect(~b2t), inf),
	\root, Pfunc({~root}),
	\pan, Pwhite(-0.5, 0.5, inf),
	\amp, 0.1,
).play(quant: ~quant);
)
~arp.stop;

(
~lead.stop;
~lead = Pbind(
    \instrument, \arp,
	\scale, Pfunc({~scale}),
	\attack, 0.1,
	\release, 0.5,
	\degree, Pseq([
	], inf),
	\dur, Pseq([
		1/4, 1/4, 1/2, 1/2
	].collect(~b2t), inf),
	\root, Pfunc({~root + 12}),
	\amp, 0.1,
).play(quant: 1);
)
~lead.stop;

(
SynthDef(\kick, {
    |out = 0, pan = 0, amp = 0.3, freq = 260|
    var body, bodyFreq, bodyAmp;
    var pop, popFreq, popAmp;
    var click, clickAmp;
    var snd;

    bodyFreq = EnvGen.ar(Env([freq, freq / 2, freq / 4], [0.035, 0.08], curve: \exp));
    bodyAmp = EnvGen.ar(Env.linen(0.005, 0.1, 0.2), doneAction: 2);
    body = SinOsc.ar(bodyFreq) * bodyAmp;

    popFreq = XLine.kr(freq * 2, freq, 0.02);
    popAmp = EnvGen.ar(Env.linen(0.001, 0.02, 0.001)) * 0.15;
    pop = SinOsc.ar(popFreq) * popAmp;

	clickAmp = EnvGen.ar(Env.perc(0.001, 0.01)) * 0.15;
    click = LPF.ar(Formant.ar(910, 4760, 2110), 3140) * clickAmp;

    snd = body + pop + click;
    snd = snd.tanh;

    Out.ar(out, Pan2.ar(snd, pan, amp));
}).add;
)

(
~kick.stop;
~kick = Pbind(
    \instrument, \kick,
	\dur, Pseq([
		1/4, 1/4, 1/4, 1/8
	].collect(~b2t), inf),
	\degree, Pseq([
		0, 2, 4,
	], inf),
	\scale, Pfunc({~scale}),
	\root, Pfunc({~root}),
	\amp, 0.1,
).play(quant: ~quant);
)
~kick.stop;
