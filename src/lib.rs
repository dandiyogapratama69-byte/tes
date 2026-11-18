// Flipbook Timeline Plugin untuk ProCraft
// Layer != Frame. Timeline bawaan OFF, Flipbook ON.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clip {
    pub start_frame: i64,
    pub end_frame: i64,
    pub source_layer_id: String,
    pub phoneme: Option<String>,
    pub label: String,
}

impl Clip {
    pub fn is_active_at(&self, frame: i64) -> bool {
        frame >= self.start_frame && frame <= self.end_frame
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrackKind {
    Normal,
    PhonemeSwitch,
    Audio,
    Text,
    Video,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlipbookTrack {
    pub id: String,
    pub name: String,
    pub kind: TrackKind,
    pub blend: String,
    pub opacity: f32,
    pub locked: bool,
    pub clips: Vec<Clip>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlipbookDoc {
    pub fps: i64,
    pub tracks: Vec<FlipbookTrack>,
    pub playhead: i64,
    pub native_timeline_hidden: bool,
}

impl FlipbookDoc {
    pub fn new_from_procraft_layers(layer_ids: Vec<(String, String)>) -> Self {
        let mut tracks = Vec::new();

        // Semua layer langsung jadi Frame 1
        for (id, name) in layer_ids {
            tracks.push(FlipbookTrack {
                id: format!("track-{}", id),
                name,
                kind: TrackKind::Normal,
                blend: "Normal".into(),
                opacity: 1.0,
                locked: false,
                clips: vec![Clip {
                    start_frame: 1,
                    end_frame: 100,
                    source_layer_id: id,
                    phoneme: None,
                    label: "Hold dari Frame 1".into(),
                }],
            });
        }

        // Group Phoneme - Track SWITCH
        tracks.push(FlipbookTrack {
            id: "track-phoneme".into(),
            name: "Group Phoneme (Mulut)".into(),
            kind: TrackKind::PhonemeSwitch,
            blend: "Normal".into(),
            opacity: 1.0,
            locked: false,
            clips: vec![
                Clip { start_frame: 1, end_frame: 5, source_layer_id: "layer-mulut-m".into(), phoneme: Some("M".into()), label: "M".into() },
                Clip { start_frame: 6, end_frame: 11, source_layer_id: "layer-mulut-a".into(), phoneme: Some("A".into()), label: "A".into() },
                Clip { start_frame: 12, end_frame: 17, source_layer_id: "layer-mulut-o".into(), phoneme: Some("O".into()), label: "O".into() },
            ],
        });

        Self { fps: 24, tracks, playhead: 1, native_timeline_hidden: true }
    }

    // Cari layer apa yang aktif di frame ini
    pub fn active_layers_at(&self, frame: i64) -> Vec<String> {
        let mut out = Vec::new();
        for t in &self.tracks {
            for c in &t.clips {
                if c.is_active_at(frame) {
                    out.push(c.source_layer_id.clone());
                }
            }
        }
        out
    }

    // Ganti phoneme di rentang frame, yang lama auto mati
    pub fn switch_phoneme(&mut self, start: i64, end: i64, phoneme: &str) {
        if let Some(t) = self.tracks.iter_mut().find(|t| t.kind == TrackKind::PhonemeSwitch) {
            t.clips.retain(|c| c.end_frame < start || c.start_frame > end);
            t.clips.push(Clip {
                start_frame: start,
                end_frame: end,
                source_layer_id: format!("layer-mulut-{}", phoneme.to_lowercase()),
                phoneme: Some(phoneme.into()),
                label: phoneme.into(),
            });
            t.clips.sort_by_key(|c| c.start_frame);
        }
    }

    // Perpanjang blok layer biasa
    pub fn extend_clip(&mut self, track_id: &str, new_end: i64) {
        if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
            if let Some(c) = t.clips.first_mut() {
                c.end_frame = new_end;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn flipbook_on_enable() -> i32 { 1 }

#[no_mangle]
pub extern "C" fn flipbook_on_disable() -> i32 { 1 }

#[no_mangle]
pub extern "C" fn flipbook_active_layers_json(frame: i64) -> i32 {
    let _ = frame;
    1
}
