// Flipbook Timeline - sesuai ABI resmi ProCraft
// Export wajib: memory + procraft_run
// Import wajib: hanya dari host "procraft", NO WASI

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    pub frame: i64,
    pub action: Option<String>,
    pub phoneme: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub fps: i64,
    pub playhead: i64,
    pub native_timeline_hidden: bool,
    pub active_layers: Vec<String>,
    pub tracks: Vec<FlipbookTrack>,
}

// Buffer output, biar memory ke-export dan nggak dibuang compiler
static mut OUT_PTR: u32 = 0;
static mut OUT_LEN: u32 = 0;
static mut OUT_BUF: [u8; 65536] = [0; 65536];

fn build_doc(frame: i64) -> Output {
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

// --- ABI RESMI PROCRAFT ---
#[no_mangle]
pub extern "C" fn procraft_run(input_ptr: u32, input_len: u32) -> u64 {
    // Baca input JSON dari memory
    let input_bytes = unsafe {
        std::slice::from_raw_parts(input_ptr as *const u8, input_len as usize)
    };
    let input: Input = serde_json::from_slice(input_bytes).unwrap_or(Input {
        frame: 1, action: None, phoneme: None, start: None, end: None,
    });

    let out = build_doc(input.frame);
    let json = serde_json::to_vec(&out).unwrap_or_else(|_| b"{}".to_vec());

    unsafe {
        let len = json.len().min(OUT_BUF.len());
        OUT_BUF[..len].copy_from_slice(&json[..len]);
        OUT_PTR = OUT_BUF.as_ptr() as u32;
        OUT_LEN = len as u32;
        // Return: (ptr << 32) | len, cara resmi balikin string dari WASM
        ((OUT_PTR as u64) << 32) | (OUT_LEN as u64)
    }
}

// Biar memory ke-export jelas, jangan dibuang compiler
#[no_mangle]
pub extern "C" fn procraft_version() -> u32 { 1 }
