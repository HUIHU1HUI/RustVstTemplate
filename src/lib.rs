//A template for creating VST audio effect plugins in rust. Includes template code for creating parameters, processing audio, and creating a GUI window

extern crate vst;
use core::ffi::c_void;
use std::sync::Arc;
use vst::{
    //api::Supported,
    buffer::AudioBuffer,
    editor::Editor,
    plugin::{Category, HostCallback, Info, Plugin, PluginParameters},
    plugin_main,
    util::AtomicFloat,
};
use vst_window::{setup, EventSource, WindowEvent};
//ACCESS FROM AUDIO THREAD.  PARAMETERS ARE WRAPPED ARC<> TO BE ACCESSED BY BOTH THREADS
struct RustVstTemplate {
    params: Arc<RustVstTemplateParameters>,
    editor_placeholder: Option<MyPluginEditor>,
}

//VST-RS REQUIRES IMPL DEFAULT
impl Default for RustVstTemplate {
    fn default() -> RustVstTemplate {
        RustVstTemplate {
            editor_placeholder: Some(MyPluginEditor::default()),
            params: Arc::new(RustVstTemplateParameters::default()),
        }
    }
}
//PARAMETER SETUP AND IMPL DEFAULT
struct RustVstTemplateParameters {
    param1: AtomicFloat,
    param2: AtomicFloat,
}

impl Default for RustVstTemplateParameters {
    fn default() -> RustVstTemplateParameters {
        RustVstTemplateParameters {
            param1: AtomicFloat::new(0.5),
            param2: AtomicFloat::new(0.5),
        }
    }
}
//PARAMETER SETUP AND FETCH
impl PluginParameters for RustVstTemplateParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.param1.get(),
            1 => self.param2.get(),
            _ => 0.0,
        }
    }
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.param1.set(val),
            1 => self.param2.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", (self.param1.get() - 0.5) * 2f32),
            1 => format!("{:.2}", (self.param2.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "parameter1",
            1 => "parameter2",
            _ => "",
        }
        .to_string()
    }
}
//MAIN PLUGIN IMPL.  PLUGIN SETUP AND DSP CODE GOES HERE
impl Plugin for RustVstTemplate {
    fn get_info(&self) -> Info {
        Info {
            name: "RUSTVSTTEMPLATE".to_string(),
            vendor: "Elastic Dummy".to_string(),
            unique_id: 1337,
            inputs: 2,
            outputs: 2,
            parameters: 2,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self {
        Self {
            editor_placeholder: Some(MyPluginEditor::default()),
            params: Arc::new(RustVstTemplateParameters::default()),
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        self.editor_placeholder
            .take()
            .map(|editor| Box::new(editor) as Box<dyn Editor>)
    }
    //GET PARAMETER INFORMATION FROM UI THREAD
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
    //AUDIO LOOP FOR PROCESSING SAMPLES
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        //ITERATE THROUGH CHANNELS
        for (input, output) in buffer.zip() {
            //ITERATE THROUGH SAMPLES IN BUFFER
            for (in_sample, out_sample) in input.into_iter().zip(output.into_iter()) {
                //DSP CODE GOES HERE
                *out_sample = *in_sample * self.params.param1.get();
            }
        }
    }
}
//GUI WINDOW SETUP AND IMPL
#[derive(Default)]
struct MyPluginEditor {
    renderer: Option<MyRenderer>,
    window_events: Option<EventSource>,
}

const WINDOW_DIMENSIONS: (i32, i32) = (500, 200);

impl Editor for MyPluginEditor {
    fn size(&self) -> (i32, i32) {
        (WINDOW_DIMENSIONS.0 as i32, WINDOW_DIMENSIONS.1 as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        if self.window_events.is_none() {
            let (window_handle, event_source) = setup(parent, WINDOW_DIMENSIONS);
            self.renderer = Some(MyRenderer::new(window_handle));
            self.window_events = Some(event_source);
            true
        } else {
            false
        }
    }

    fn is_open(&mut self) -> bool {
        self.window_events.is_some()
    }

    fn close(&mut self) {
        drop(self.renderer.take());
        drop(self.window_events.take());
    }

    fn idle(&mut self) {
        if let Some(window_events) = &mut self.window_events {
            while let Some(event) = window_events.poll_event() {
                match event {
                    WindowEvent::MouseClick(_) => println!("Click!"),
                    _ => (),
                }
            }
        }
        if let Some(renderer) = &mut self.renderer {
            renderer.draw_frame();
        }
    }
}
//WINDOW RENDERER
struct MyRenderer;

impl MyRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(_handle: W) -> Self {
        Self
    }
    pub fn draw_frame(&mut self) {
        //RENDER CODE HERE
    }
}
//EVERY VST MUST CALL PLUGIN_MAIN! MACRO
plugin_main!(RustVstTemplate);
