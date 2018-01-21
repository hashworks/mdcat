// Copyright 2018 Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tools for syntax highlighting.

use std::io::{Result, Write};
use syntect::highlighting::{FontStyle, Style};
use termion::{color, style};

/// Write regions as ANSI 8-bit coloured text.
///
/// We use this function to simplify syntax highlighting to 8-bit ANSI values
/// which every theme provides.  Contrary to 24 bit colours this gives us a good
/// guarantee that highlighting works with any terminal colour theme, whether
/// light or dark, and saves us all the hassle of mismatching colours.
///
/// We assume Solarized colours here: Solarized cleanly maps to 8-bit ANSI
/// colours so we can safely map its RGB colour values back to ANSI colours.  We
/// do so for all accent colours, but leave "base*" colours alone: Base colours
/// change depending on light or dark Solarized; to address both light and dark
/// backgrounds we must map all base colours to the default terminal colours.
///
/// Furthermore we completely ignore any background colour settings, to avoid
/// conflicts with the terminal colour theme.s
pub fn write_as_ansi<W: Write>(writer: &mut W, regions: &[(Style, &str)]) -> Result<()> {
    for &(style, text) in regions {
        let rgb = {
            let fg = style.foreground;
            (fg.r, fg.g, fg.b)
        };
        match rgb {
            (0x00, 0x2b, 0x36) => write!(writer, "{}", color::Fg(color::Reset))?, // base03
            (0x07, 0x36, 0x42) => write!(writer, "{}", color::Fg(color::Reset))?, // base02
            (0x58, 0x6e, 0x75) => write!(writer, "{}", color::Fg(color::Reset))?, // base01
            (0x65, 0x7b, 0x83) => write!(writer, "{}", color::Fg(color::Reset))?, // base00
            (0x83, 0x94, 0x96) => write!(writer, "{}", color::Fg(color::Reset))?, // base0
            (0x93, 0xa1, 0xa1) => write!(writer, "{}", color::Fg(color::Reset))?, // base1
            (0xee, 0xe8, 0xd5) => write!(writer, "{}", color::Fg(color::Reset))?, // base2
            (0xfd, 0xf6, 0xe3) => write!(writer, "{}", color::Fg(color::Reset))?, // base3
            (0xb5, 0x89, 0x00) => write!(writer, "{}", color::Fg(color::Yellow))?, // yellow
            (0xcb, 0x4b, 0x16) => write!(writer, "{}", color::Fg(color::LightRed))?, // orange
            (0xdc, 0x32, 0x2f) => write!(writer, "{}", color::Fg(color::Red))?,   // red
            (0xd3, 0x36, 0x82) => write!(writer, "{}", color::Fg(color::Magenta))?, // magenta
            (0x6c, 0x71, 0xc4) => write!(writer, "{}", color::Fg(color::LightMagenta))?, // violet
            (0x26, 0x8b, 0xd2) => write!(writer, "{}", color::Fg(color::Blue))?,  // blue
            (0x2a, 0xa1, 0x98) => write!(writer, "{}", color::Fg(color::Cyan))?,  // cyan
            (0x85, 0x99, 0x00) => write!(writer, "{}", color::Fg(color::Green))?, // green
            (r, g, b) => panic!("Unexpected RGB colour: #{:2>0x}{:2>0x}{:2>0x}", r, g, b),
        };
        let font = style.font_style;
        if font.contains(FontStyle::BOLD) {
            write!(writer, "{}", style::Bold)?;
        } else {
            write!(writer, "{}", style::NoBold)?;
        }
        if font.contains(FontStyle::ITALIC) {
            write!(writer, "{}", style::Italic)?;
        } else {
            write!(writer, "{}", style::NoItalic)?;
        }
        if font.contains(FontStyle::UNDERLINE) {
            write!(writer, "{}", style::Underline)?;
        } else {
            write!(writer, "{}", style::NoUnderline)?;
        }
        write!(writer, "{}{}", text, style::Reset)?;
    }
    Ok(())
}