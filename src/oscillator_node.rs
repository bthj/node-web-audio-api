// ---------------------------------------------------------- //
// ---------------------------------------------------------- //
//    - WARNING - DO NOT EDIT                               - //
//    - This file has been generated                        - //
// ---------------------------------------------------------- //
// ---------------------------------------------------------- //

use crate::*;
use napi::*;
use napi_derive::js_function;
use std::rc::Rc;
use web_audio_api::node::*;

pub(crate) struct NapiOscillatorNode(Rc<OscillatorNode>);

impl NapiOscillatorNode {
    pub fn create_js_class(env: &Env) -> Result<JsFunction> {
        env.define_class(
            "OscillatorNode",
            constructor,
            &[
                // Attributes
                Property::new("type")?
                    .with_getter(get_type)
                    .with_setter(set_type)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                // Methods
                Property::new("setPeriodicWave")?
                    .with_method(set_periodic_wave)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                // AudioNode interface
                Property::new("channelCount")?
                    .with_getter(get_channel_count)
                    .with_setter(set_channel_count),
                Property::new("channelCountMode")?
                    .with_getter(get_channel_count_mode)
                    .with_setter(set_channel_count_mode),
                Property::new("channelInterpretation")?
                    .with_getter(get_channel_interpretation)
                    .with_setter(set_channel_interpretation),
                Property::new("connect")?
                    .with_method(connect)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                Property::new("disconnect")?
                    .with_method(disconnect)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                // AudioScheduledSourceNode interface
                Property::new("start")?
                    .with_method(start)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                Property::new("stop")?
                    .with_method(stop)
                    .with_property_attributes(PropertyAttributes::Enumerable),
            ],
        )
    }

    pub fn unwrap(&self) -> &OscillatorNode {
        &self.0
    }
}

#[js_function(2)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let mut js_this = ctx.this_unchecked::<JsObject>();

    // first argument is always AudioContext
    let js_audio_context = ctx.get::<JsObject>(0)?;

    js_this.define_properties(&[
        Property::new("context")?
            .with_value(&js_audio_context)
            .with_property_attributes(PropertyAttributes::Enumerable),
        // this must be put on the instance and not in the prototype to be reachable
        Property::new("Symbol.toStringTag")?
            .with_value(&ctx.env.create_string("OscillatorNode")?)
            .with_property_attributes(PropertyAttributes::Static),
    ])?;

    // parse options
    let options = match ctx.try_get::<JsObject>(1)? {
        Either::A(options_js) => {
            let some_type_js = options_js.get::<&str, JsString>("type")?;
            let type_ = if let Some(type_js) = some_type_js {
                let type_str = type_js.into_utf8()?.into_owned()?;

                match type_str.as_str() {
                    "sine" => OscillatorType::Sine,
                    "square" => OscillatorType::Square,
                    "sawtooth" => OscillatorType::Sawtooth,
                    "triangle" => OscillatorType::Triangle,
                    "custom" => OscillatorType::Custom,
                    _ => panic!("undefined value for OscillatorType"),
                }
            } else {
                OscillatorType::default()
            };

            let some_frequency_js = options_js.get::<&str, JsNumber>("frequency")?;
            let frequency = if let Some(frequency_js) = some_frequency_js {
                frequency_js.get_double()? as f32
            } else {
                440.
            };

            let some_detune_js = options_js.get::<&str, JsNumber>("detune")?;
            let detune = if let Some(detune_js) = some_detune_js {
                detune_js.get_double()? as f32
            } else {
                0.
            };

            let some_periodic_wave_js = options_js.get::<&str, JsObject>("periodicWave")?;
            let periodic_wave = if let Some(periodic_wave_js) = some_periodic_wave_js {
                let periodic_wave_napi = ctx.env.unwrap::<NapiPeriodicWave>(&periodic_wave_js)?;
                Some(periodic_wave_napi.unwrap().clone())
            } else {
                None
            };

            let node_defaults = OscillatorOptions::default();
            let channel_config_defaults = node_defaults.channel_config;

            let some_channel_count_js = options_js.get::<&str, JsNumber>("channelCount")?;
            let channel_count = if let Some(channel_count_js) = some_channel_count_js {
                channel_count_js.get_double()? as usize
            } else {
                channel_config_defaults.count
            };

            let some_channel_count_mode_js =
                options_js.get::<&str, JsString>("channelCountMode")?;
            let channel_count_mode = if let Some(channel_count_mode_js) = some_channel_count_mode_js
            {
                let channel_count_mode_str = channel_count_mode_js.into_utf8()?.into_owned()?;

                match channel_count_mode_str.as_str() {
                    "max" => ChannelCountMode::Max,
                    "clamped-max" => ChannelCountMode::ClampedMax,
                    "explicit" => ChannelCountMode::Explicit,
                    _ => panic!("undefined value for ChannelCountMode"),
                }
            } else {
                channel_config_defaults.count_mode
            };

            let some_channel_interpretation_js =
                options_js.get::<&str, JsString>("channelInterpretation")?;
            let channel_interpretation =
                if let Some(channel_interpretation_js) = some_channel_interpretation_js {
                    let channel_interpretation_str =
                        channel_interpretation_js.into_utf8()?.into_owned()?;

                    match channel_interpretation_str.as_str() {
                        "speakers" => ChannelInterpretation::Speakers,
                        "discrete" => ChannelInterpretation::Discrete,
                        _ => panic!("undefined value for ChannelInterpretation"),
                    }
                } else {
                    channel_config_defaults.interpretation
                };

            OscillatorOptions {
                type_,
                frequency,
                detune,
                periodic_wave,
                channel_config: ChannelConfigOptions {
                    count: channel_count,
                    count_mode: channel_count_mode,
                    interpretation: channel_interpretation,
                },
            }
        }
        Either::B(_) => Default::default(),
    };

    // create native node
    let audio_context_name =
        js_audio_context.get_named_property::<JsString>("Symbol.toStringTag")?;
    let audio_context_utf8_name = audio_context_name.into_utf8()?.into_owned()?;
    let audio_context_str = &audio_context_utf8_name[..];

    let native_node = match audio_context_str {
        "AudioContext" => {
            let napi_audio_context = ctx.env.unwrap::<NapiAudioContext>(&js_audio_context)?;
            let audio_context = napi_audio_context.unwrap();
            Rc::new(OscillatorNode::new(audio_context, options))
        }
        "OfflineAudioContext" => {
            let napi_audio_context = ctx
                .env
                .unwrap::<NapiOfflineAudioContext>(&js_audio_context)?;
            let audio_context = napi_audio_context.unwrap();
            Rc::new(OscillatorNode::new(audio_context, options))
        }
        &_ => panic!("not supported"),
    };

    // AudioParam: OscillatorNode::frequency
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::OscillatorNodeFrequency(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("frequency", &js_obj)?;

    // AudioParam: OscillatorNode::detune
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::OscillatorNodeDetune(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("detune", &js_obj)?;

    // finalize instance creation
    let napi_node = NapiOscillatorNode(native_node);
    ctx.env.wrap(&mut js_this, napi_node)?;

    ctx.env.get_undefined()
}

// -------------------------------------------------
// AudioNode Interface
// -------------------------------------------------
#[js_function]
fn get_channel_count(ctx: CallContext) -> Result<JsNumber> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let channel_count = node.channel_count() as f64;

    ctx.env.create_double(channel_count)
}

#[js_function(1)]
fn set_channel_count(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let channel_count = ctx.get::<JsNumber>(0)?.get_double()? as usize;
    node.set_channel_count(channel_count);

    ctx.env.get_undefined()
}

#[js_function]
fn get_channel_count_mode(ctx: CallContext) -> Result<JsString> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let value = node.channel_count_mode();
    let value_str = match value {
        ChannelCountMode::Max => "max",
        ChannelCountMode::ClampedMax => "clamped-max",
        ChannelCountMode::Explicit => "explicit",
    };

    ctx.env.create_string(value_str)
}

#[js_function(1)]
fn set_channel_count_mode(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let js_str = ctx.get::<JsString>(0)?;
    let uf8_str = js_str.into_utf8()?.into_owned()?;
    let value = match uf8_str.as_str() {
        "max" => ChannelCountMode::Max,
        "clamped-max" => ChannelCountMode::ClampedMax,
        "explicit" => ChannelCountMode::Explicit,
        _ => panic!("undefined value for ChannelCountMode"),
    };
    node.set_channel_count_mode(value);

    ctx.env.get_undefined()
}

#[js_function]
fn get_channel_interpretation(ctx: CallContext) -> Result<JsString> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let value = node.channel_interpretation();
    let value_str = match value {
        ChannelInterpretation::Speakers => "speakers",
        ChannelInterpretation::Discrete => "discrete",
    };

    ctx.env.create_string(value_str)
}

#[js_function(1)]
fn set_channel_interpretation(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let js_str = ctx.get::<JsString>(0)?;
    let uf8_str = js_str.into_utf8()?.into_owned()?;
    let value = match uf8_str.as_str() {
        "speakers" => ChannelInterpretation::Speakers,
        "discrete" => ChannelInterpretation::Discrete,
        _ => panic!("undefined value for ChannelInterpretation"),
    };
    node.set_channel_interpretation(value);

    ctx.env.get_undefined()
}

connect_method!(NapiOscillatorNode);
disconnect_method!(NapiOscillatorNode);
// disconnect_method!(NapiOscillatorNode);

// -------------------------------------------------
// AudioScheduledSourceNode Interface
// -------------------------------------------------
#[js_function(1)]
fn start(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    match ctx.length {
        0 => node.start(),
        1 => {
            let when = ctx.get::<JsNumber>(0)?.get_double()?;
            node.start_at(when);
        }
        _ => (),
    }

    ctx.env.get_undefined()
}

#[js_function(1)]
fn stop(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    match ctx.length {
        0 => node.stop(),
        1 => {
            let when = ctx.get::<JsNumber>(0)?.try_into()?;
            node.stop_at(when);
        }
        _ => (),
    };

    ctx.env.get_undefined()
}

// -------------------------------------------------
// GETTERS
// -------------------------------------------------

#[js_function(0)]
fn get_type(ctx: CallContext) -> Result<JsString> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let value = node.type_();
    let js_value = match value {
        OscillatorType::Sine => "sine",
        OscillatorType::Square => "square",
        OscillatorType::Sawtooth => "sawtooth",
        OscillatorType::Triangle => "triangle",
        OscillatorType::Custom => "custom",
    };

    ctx.env.create_string(js_value)
}

// -------------------------------------------------
// SETTERS
// -------------------------------------------------

#[js_function(1)]
fn set_type(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let js_str = ctx.get::<JsString>(0)?;
    let uf8_str = js_str.into_utf8()?.into_owned()?;
    let value = match uf8_str.as_str() {
        "sine" => OscillatorType::Sine,
        "square" => OscillatorType::Square,
        "sawtooth" => OscillatorType::Sawtooth,
        "triangle" => OscillatorType::Triangle,
        "custom" => OscillatorType::Custom,
        _ => panic!("undefined value for OscillatorType"),
    };

    node.set_type(value);

    ctx.env.get_undefined()
}

// -------------------------------------------------
// METHODS
// -------------------------------------------------

#[js_function(1)]
fn set_periodic_wave(ctx: CallContext) -> Result<JsUndefined> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiOscillatorNode>(&js_this)?;
    // avoid warnings while we don't support all methods
    #[allow(unused_variables)]
    let node = napi_node.unwrap();

    let periodic_wave_js = ctx.get::<JsObject>(0)?;
    let periodic_wave_napi = ctx.env.unwrap::<NapiPeriodicWave>(&periodic_wave_js)?;
    let periodic_wave = periodic_wave_napi.unwrap().clone();

    node.set_periodic_wave(periodic_wave);

    ctx.env.get_undefined()
}
