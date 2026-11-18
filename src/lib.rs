// Flipbook Timeline - ABI resmi ProCraft
// Export wajib: memory + procraft_run(ptr, len) -> packed (ptr<<32 | len)

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clip {
    pub start_frame: i64,
    pub end_frame: i64,
    pub source_layer_id: String,
    pub phoneme: Option<String>,
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlipbookTrack {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub clips: Vec<Clip>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Input {
    #[serde(default)]
    pub frame: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub fps: i64,
    pub playhead: i64,
    pub native_timeline_hidden: bool,
    pub active_layers: Vec<String>,
    pub tracks: Vec<FlipbookTrack>,
}

static mut OUT_BUF: [u8; 65536] = [0; 65536];

fn build_doc(frame: i64) -> Output {
    let frame = if frame < 1 { 1 } else { frame };
    let tracks = vec![
        FlipbookTrack {
            id: "track-karakter".into(),
            name: "Karakter".into(),
            kind: "Normal".into(),
            clips: vec![Clip {
                start_frame: 1, end_frame: 100,
                source_layer_id: "layer-karakter".into(),
                phoneme: None, label: "Hold dari Frame 1".into(),
            }],
        },
        FlipbookTrack {
            id: "track-phoneme".into(),
            name: "Group Phoneme (Mulut)".into(),
            kind: "PhonemeSwitch".into(),
            clips: vec![
                Clip { start_frame: 1, end_frame: 5, source_layer_id: "layer-mulut-m".into(), phoneme: Some("M".into()), label: "M".into() },
                Clip { start_frame: 6, end_frame: 11, source_layer_id: "layer-mulut-a".into(), phoneme: Some("A".into()), label: "A".into() },
                Clip { start_frame: 12, end_frame: 17, source_layer_id: "layer-mulut-o".into(), phoneme: Some("O".into()), label: "O".into() },
            ],
        },
    ];

    let mut active = Vec::new();
    for t in &tracks {
        for c in &t.clips {
            if frame >= c.start_frame && frame <= c.end_frame {
                active.push(c.source_layer_id.clone());
            }
        }
    }

    Output { fps: 24, playhead: frame, native_timeline_hidden: true, active_layers: active, tracks }
}

#[no_mangle]
pub extern "C" fn procraft_run(input_ptr: u32, input_len: u32) -> u64 {
    let frame: i64 = if input_ptr == 0 || input_len == 0 || input_len > 65536 {
        1
    } else {
        let bytes = unsafe {
            core::slice::from_raw_parts(input_ptr as *const u8, input_len as usize)
        };
        match serde_json::from_slice::<Input>(bytes) {
            Ok(v) => if v.frame < 1 { 1 } else { v.frame },
            Err(_) => 1,
        }
    };

    let out = build_doc(frame);
    let json: Vec<u8> = match serde_json::to_vec(&out) {
        Ok(v) => v,
        Err(_) => b"{}".to_vec(),
    };

    unsafe {
        let len = json.len().min(OUT_BUF.len());
        OUT_BUF[..len].copy_from_slice(&json[..len]);
        let ptr = OUT_BUF.as_ptr() as u64;
        (ptr << 32) | (len as u64)
    }
}

#[no_mangle]
pub extern "C" fn procraft_version() -> u32 { 3 }
