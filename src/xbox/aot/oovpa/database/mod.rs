//! Rust mirror of Cxbx-R's XbSymbolDatabase layout.
//!
//! These modules are generated as `database/<library>/xdkNNNN.rs` from the
//! matching Cxbx-R `.inl` sources. Each `xdkNNNN.rs` table is cumulative for
//! that SDK build: it contains the richest pattern revision from source files
//! whose version is less than or equal to NNNN.
//! The older flat `xbsymdb_*` blobs are intentionally left in-tree as backup
//! data, but this module is the scanner-facing source of truth.

use super::{OovpaPattern, OovpaPatternMeta, OovpaXref};

pub mod d3d8;
pub mod d3d8_ltcg;
pub mod dsound;
pub mod xapi;
pub mod xgraphic;

pub fn patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    d3d8_patterns_for_xdk(xdk)
}

pub fn d3d8_patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    match xdk {
        3911 => d3d8::xdk3911::PATTERNS,
        3925 => d3d8::xdk3925::PATTERNS,
        3936 => d3d8::xdk3936::PATTERNS,
        3947 => d3d8::xdk3947::PATTERNS,
        3950 => d3d8::xdk3950::PATTERNS,
        4034 => d3d8::xdk4034::PATTERNS,
        4039 => d3d8::xdk4039::PATTERNS,
        4134 => d3d8::xdk4134::PATTERNS,
        4242 => d3d8::xdk4242::PATTERNS,
        4361 => d3d8::xdk4361::PATTERNS,
        4432 => d3d8::xdk4432::PATTERNS,
        4531 => d3d8::xdk4531::PATTERNS,
        4627 => d3d8::xdk4627::PATTERNS,
        4721 => d3d8::xdk4721::PATTERNS,
        4831 => d3d8::xdk4831::PATTERNS,
        4928 => d3d8::xdk4928::PATTERNS,
        5028 => d3d8::xdk5028::PATTERNS,
        5120 => d3d8::xdk5120::PATTERNS,
        5233 => d3d8::xdk5233::PATTERNS,
        5344 => d3d8::xdk5344::PATTERNS,
        5455 => d3d8::xdk5455::PATTERNS,
        5558 => d3d8::xdk5558::PATTERNS,
        5788 => d3d8::xdk5788::PATTERNS,
        5849 => d3d8::xdk5849::PATTERNS,
        _ => &[],
    }
}

pub fn d3d8_ltcg_patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    match xdk {
        3911 => d3d8_ltcg::xdk3911::PATTERNS,
        3925 => d3d8_ltcg::xdk3925::PATTERNS,
        3936 => d3d8_ltcg::xdk3936::PATTERNS,
        3947 => d3d8_ltcg::xdk3947::PATTERNS,
        3950 => d3d8_ltcg::xdk3950::PATTERNS,
        4034 => d3d8_ltcg::xdk4034::PATTERNS,
        4039 => d3d8_ltcg::xdk4039::PATTERNS,
        4134 => d3d8_ltcg::xdk4134::PATTERNS,
        4242 => d3d8_ltcg::xdk4242::PATTERNS,
        4361 => d3d8_ltcg::xdk4361::PATTERNS,
        4432 => d3d8_ltcg::xdk4432::PATTERNS,
        4531 => d3d8_ltcg::xdk4531::PATTERNS,
        4627 => d3d8_ltcg::xdk4627::PATTERNS,
        4721 => d3d8_ltcg::xdk4721::PATTERNS,
        4831 => d3d8_ltcg::xdk4831::PATTERNS,
        4928 => d3d8_ltcg::xdk4928::PATTERNS,
        5028 => d3d8_ltcg::xdk5028::PATTERNS,
        5120 => d3d8_ltcg::xdk5120::PATTERNS,
        5233 => d3d8_ltcg::xdk5233::PATTERNS,
        5344 => d3d8_ltcg::xdk5344::PATTERNS,
        5455 => d3d8_ltcg::xdk5455::PATTERNS,
        5558 => d3d8_ltcg::xdk5558::PATTERNS,
        5788 => d3d8_ltcg::xdk5788::PATTERNS,
        5849 => d3d8_ltcg::xdk5849::PATTERNS,
        _ => &[],
    }
}

pub fn dsound_patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    match xdk {
        3911 => dsound::xdk3911::PATTERNS,
        3925 => dsound::xdk3925::PATTERNS,
        3936 => dsound::xdk3936::PATTERNS,
        3947 => dsound::xdk3947::PATTERNS,
        3950 => dsound::xdk3950::PATTERNS,
        4034 => dsound::xdk4034::PATTERNS,
        4039 => dsound::xdk4039::PATTERNS,
        4134 => dsound::xdk4134::PATTERNS,
        4242 => dsound::xdk4242::PATTERNS,
        4361 => dsound::xdk4361::PATTERNS,
        4432 => dsound::xdk4432::PATTERNS,
        4531 => dsound::xdk4531::PATTERNS,
        4627 => dsound::xdk4627::PATTERNS,
        4721 => dsound::xdk4721::PATTERNS,
        4831 => dsound::xdk4831::PATTERNS,
        4928 => dsound::xdk4928::PATTERNS,
        5028 => dsound::xdk5028::PATTERNS,
        5120 => dsound::xdk5120::PATTERNS,
        5233 => dsound::xdk5233::PATTERNS,
        5344 => dsound::xdk5344::PATTERNS,
        5455 => dsound::xdk5455::PATTERNS,
        5558 => dsound::xdk5558::PATTERNS,
        5788 => dsound::xdk5788::PATTERNS,
        5849 => dsound::xdk5849::PATTERNS,
        _ => &[],
    }
}

pub fn xapi_patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    match xdk {
        3911 => xapi::xdk3911::PATTERNS,
        3925 => xapi::xdk3925::PATTERNS,
        3936 => xapi::xdk3936::PATTERNS,
        3947 => xapi::xdk3947::PATTERNS,
        3950 => xapi::xdk3950::PATTERNS,
        4034 => xapi::xdk4034::PATTERNS,
        4039 => xapi::xdk4039::PATTERNS,
        4134 => xapi::xdk4134::PATTERNS,
        4242 => xapi::xdk4242::PATTERNS,
        4361 => xapi::xdk4361::PATTERNS,
        4432 => xapi::xdk4432::PATTERNS,
        4531 => xapi::xdk4531::PATTERNS,
        4627 => xapi::xdk4627::PATTERNS,
        4721 => xapi::xdk4721::PATTERNS,
        4831 => xapi::xdk4831::PATTERNS,
        4928 => xapi::xdk4928::PATTERNS,
        5028 => xapi::xdk5028::PATTERNS,
        5120 => xapi::xdk5120::PATTERNS,
        5233 => xapi::xdk5233::PATTERNS,
        5344 => xapi::xdk5344::PATTERNS,
        5455 => xapi::xdk5455::PATTERNS,
        5558 => xapi::xdk5558::PATTERNS,
        5788 => xapi::xdk5788::PATTERNS,
        5849 => xapi::xdk5849::PATTERNS,
        _ => &[],
    }
}

pub fn xgraphic_patterns_for_xdk(xdk: u32) -> &'static [OovpaPattern] {
    match xdk {
        3911 => xgraphic::xdk3911::PATTERNS,
        3925 => xgraphic::xdk3925::PATTERNS,
        3936 => xgraphic::xdk3936::PATTERNS,
        3947 => xgraphic::xdk3947::PATTERNS,
        3950 => xgraphic::xdk3950::PATTERNS,
        4034 => xgraphic::xdk4034::PATTERNS,
        4039 => xgraphic::xdk4039::PATTERNS,
        4134 => xgraphic::xdk4134::PATTERNS,
        4242 => xgraphic::xdk4242::PATTERNS,
        4361 => xgraphic::xdk4361::PATTERNS,
        4432 => xgraphic::xdk4432::PATTERNS,
        4531 => xgraphic::xdk4531::PATTERNS,
        4627 => xgraphic::xdk4627::PATTERNS,
        4721 => xgraphic::xdk4721::PATTERNS,
        4831 => xgraphic::xdk4831::PATTERNS,
        4928 => xgraphic::xdk4928::PATTERNS,
        5028 => xgraphic::xdk5028::PATTERNS,
        5120 => xgraphic::xdk5120::PATTERNS,
        5233 => xgraphic::xdk5233::PATTERNS,
        5344 => xgraphic::xdk5344::PATTERNS,
        5455 => xgraphic::xdk5455::PATTERNS,
        5558 => xgraphic::xdk5558::PATTERNS,
        5788 => xgraphic::xdk5788::PATTERNS,
        5849 => xgraphic::xdk5849::PATTERNS,
        _ => &[],
    }
}

pub fn d3d8_metadata_for_xdk(xdk: u32) -> &'static [OovpaPatternMeta] {
    match xdk {
        3911 => d3d8::xdk3911::METADATA,
        3925 => d3d8::xdk3925::METADATA,
        3936 => d3d8::xdk3936::METADATA,
        3947 => d3d8::xdk3947::METADATA,
        3950 => d3d8::xdk3950::METADATA,
        4034 => d3d8::xdk4034::METADATA,
        4039 => d3d8::xdk4039::METADATA,
        4134 => d3d8::xdk4134::METADATA,
        4242 => d3d8::xdk4242::METADATA,
        4361 => d3d8::xdk4361::METADATA,
        4432 => d3d8::xdk4432::METADATA,
        4531 => d3d8::xdk4531::METADATA,
        4627 => d3d8::xdk4627::METADATA,
        4721 => d3d8::xdk4721::METADATA,
        4831 => d3d8::xdk4831::METADATA,
        4928 => d3d8::xdk4928::METADATA,
        5028 => d3d8::xdk5028::METADATA,
        5120 => d3d8::xdk5120::METADATA,
        5233 => d3d8::xdk5233::METADATA,
        5344 => d3d8::xdk5344::METADATA,
        5455 => d3d8::xdk5455::METADATA,
        5558 => d3d8::xdk5558::METADATA,
        5788 => d3d8::xdk5788::METADATA,
        5849 => d3d8::xdk5849::METADATA,
        _ => &[],
    }
}

pub fn d3d8_ltcg_metadata_for_xdk(xdk: u32) -> &'static [OovpaPatternMeta] {
    match xdk {
        3911 => d3d8_ltcg::xdk3911::METADATA,
        3925 => d3d8_ltcg::xdk3925::METADATA,
        3936 => d3d8_ltcg::xdk3936::METADATA,
        3947 => d3d8_ltcg::xdk3947::METADATA,
        3950 => d3d8_ltcg::xdk3950::METADATA,
        4034 => d3d8_ltcg::xdk4034::METADATA,
        4039 => d3d8_ltcg::xdk4039::METADATA,
        4134 => d3d8_ltcg::xdk4134::METADATA,
        4242 => d3d8_ltcg::xdk4242::METADATA,
        4361 => d3d8_ltcg::xdk4361::METADATA,
        4432 => d3d8_ltcg::xdk4432::METADATA,
        4531 => d3d8_ltcg::xdk4531::METADATA,
        4627 => d3d8_ltcg::xdk4627::METADATA,
        4721 => d3d8_ltcg::xdk4721::METADATA,
        4831 => d3d8_ltcg::xdk4831::METADATA,
        4928 => d3d8_ltcg::xdk4928::METADATA,
        5028 => d3d8_ltcg::xdk5028::METADATA,
        5120 => d3d8_ltcg::xdk5120::METADATA,
        5233 => d3d8_ltcg::xdk5233::METADATA,
        5344 => d3d8_ltcg::xdk5344::METADATA,
        5455 => d3d8_ltcg::xdk5455::METADATA,
        5558 => d3d8_ltcg::xdk5558::METADATA,
        5788 => d3d8_ltcg::xdk5788::METADATA,
        5849 => d3d8_ltcg::xdk5849::METADATA,
        _ => &[],
    }
}

pub fn dsound_metadata_for_xdk(xdk: u32) -> &'static [OovpaPatternMeta] {
    match xdk {
        3911 => dsound::xdk3911::METADATA,
        3925 => dsound::xdk3925::METADATA,
        3936 => dsound::xdk3936::METADATA,
        3947 => dsound::xdk3947::METADATA,
        3950 => dsound::xdk3950::METADATA,
        4034 => dsound::xdk4034::METADATA,
        4039 => dsound::xdk4039::METADATA,
        4134 => dsound::xdk4134::METADATA,
        4242 => dsound::xdk4242::METADATA,
        4361 => dsound::xdk4361::METADATA,
        4432 => dsound::xdk4432::METADATA,
        4531 => dsound::xdk4531::METADATA,
        4627 => dsound::xdk4627::METADATA,
        4721 => dsound::xdk4721::METADATA,
        4831 => dsound::xdk4831::METADATA,
        4928 => dsound::xdk4928::METADATA,
        5028 => dsound::xdk5028::METADATA,
        5120 => dsound::xdk5120::METADATA,
        5233 => dsound::xdk5233::METADATA,
        5344 => dsound::xdk5344::METADATA,
        5455 => dsound::xdk5455::METADATA,
        5558 => dsound::xdk5558::METADATA,
        5788 => dsound::xdk5788::METADATA,
        5849 => dsound::xdk5849::METADATA,
        _ => &[],
    }
}

pub fn xapi_metadata_for_xdk(xdk: u32) -> &'static [OovpaPatternMeta] {
    match xdk {
        3911 => xapi::xdk3911::METADATA,
        3925 => xapi::xdk3925::METADATA,
        3936 => xapi::xdk3936::METADATA,
        3947 => xapi::xdk3947::METADATA,
        3950 => xapi::xdk3950::METADATA,
        4034 => xapi::xdk4034::METADATA,
        4039 => xapi::xdk4039::METADATA,
        4134 => xapi::xdk4134::METADATA,
        4242 => xapi::xdk4242::METADATA,
        4361 => xapi::xdk4361::METADATA,
        4432 => xapi::xdk4432::METADATA,
        4531 => xapi::xdk4531::METADATA,
        4627 => xapi::xdk4627::METADATA,
        4721 => xapi::xdk4721::METADATA,
        4831 => xapi::xdk4831::METADATA,
        4928 => xapi::xdk4928::METADATA,
        5028 => xapi::xdk5028::METADATA,
        5120 => xapi::xdk5120::METADATA,
        5233 => xapi::xdk5233::METADATA,
        5344 => xapi::xdk5344::METADATA,
        5455 => xapi::xdk5455::METADATA,
        5558 => xapi::xdk5558::METADATA,
        5788 => xapi::xdk5788::METADATA,
        5849 => xapi::xdk5849::METADATA,
        _ => &[],
    }
}

pub fn xgraphic_metadata_for_xdk(xdk: u32) -> &'static [OovpaPatternMeta] {
    match xdk {
        3911 => xgraphic::xdk3911::METADATA,
        3925 => xgraphic::xdk3925::METADATA,
        3936 => xgraphic::xdk3936::METADATA,
        3947 => xgraphic::xdk3947::METADATA,
        3950 => xgraphic::xdk3950::METADATA,
        4034 => xgraphic::xdk4034::METADATA,
        4039 => xgraphic::xdk4039::METADATA,
        4134 => xgraphic::xdk4134::METADATA,
        4242 => xgraphic::xdk4242::METADATA,
        4361 => xgraphic::xdk4361::METADATA,
        4432 => xgraphic::xdk4432::METADATA,
        4531 => xgraphic::xdk4531::METADATA,
        4627 => xgraphic::xdk4627::METADATA,
        4721 => xgraphic::xdk4721::METADATA,
        4831 => xgraphic::xdk4831::METADATA,
        4928 => xgraphic::xdk4928::METADATA,
        5028 => xgraphic::xdk5028::METADATA,
        5120 => xgraphic::xdk5120::METADATA,
        5233 => xgraphic::xdk5233::METADATA,
        5344 => xgraphic::xdk5344::METADATA,
        5455 => xgraphic::xdk5455::METADATA,
        5558 => xgraphic::xdk5558::METADATA,
        5788 => xgraphic::xdk5788::METADATA,
        5849 => xgraphic::xdk5849::METADATA,
        _ => &[],
    }
}

pub fn xrefs_for_pattern(pat: &OovpaPattern, xdk: u32) -> &'static [OovpaXref] {
    metadata_for_pattern(pat, xdk)
        .map(|meta| meta.xrefs)
        .unwrap_or(&[])
}

pub fn source_for_pattern(pat: &OovpaPattern, xdk: u32) -> Option<&'static str> {
    metadata_for_pattern(pat, xdk).map(|meta| meta.source_file)
}

fn metadata_for_pattern(pat: &OovpaPattern, xdk: u32) -> Option<&'static OovpaPatternMeta> {
    let tables: [&'static [OovpaPatternMeta]; 5] = [
        d3d8_ltcg_metadata_for_xdk(xdk),
        d3d8_metadata_for_xdk(xdk),
        dsound_metadata_for_xdk(xdk),
        xapi_metadata_for_xdk(xdk),
        xgraphic_metadata_for_xdk(xdk),
    ];

    tables
        .iter()
        .flat_map(|table| table.iter())
        .find(|meta| meta.name == pat.name && meta.min_version == pat.min_version)
}
