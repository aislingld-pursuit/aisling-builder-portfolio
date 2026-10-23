//! The site palette, as data with its invariants attached.
//!
//! Carried over from the L2 portfolio: `contrast_x100` is a `const fn`, so a colour
//! pairing that falls below WCAG on its own background is a compile error rather than
//! something a person has to notice. The light theme is the lab notebook (warm paper,
//! ink, oxide accent); the dark theme is the terminal (graphite, paper-white, ember).

/// A colour, kept as three channels so contrast can be computed from it.
#[derive(Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

// ── compile-time contrast ────────────────────────────────────────────────────
//
// WCAG needs sRGB channels linearised by `((c+0.055)/1.055)^2.4`, and `powf` is not
// available in a const fn. So the curve is precomputed once as fixed-point integers
// (linear value x 1_000_000) and everything downstream is integer arithmetic.

#[rustfmt::skip]
const SRGB_LINEAR: [u32; 256] = [
          0,     304,     607,     911,    1214,    1518,    1821,    2125,
       2428,    2732,    3035,    3347,    3677,    4025,    4391,    4777,
       5182,    5605,    6049,    6512,    6995,    7499,    8023,    8568,
       9134,    9721,   10330,   10960,   11612,   12286,   12983,   13702,
      14444,   15209,   15996,   16807,   17642,   18500,   19382,   20289,
      21219,   22174,   23153,   24158,   25187,   26241,   27321,   28426,
      29557,   30713,   31896,   33105,   34340,   35601,   36889,   38204,
      39546,   40915,   42311,   43735,   45186,   46665,   48172,   49707,
      51269,   52861,   54480,   56128,   57805,   59511,   61246,   63010,
      64803,   66626,   68478,   70360,   72272,   74214,   76185,   78187,
      80220,   82283,   84376,   86500,   88656,   90842,   93059,   95307,
      97587,   99899,  102242,  104616,  107023,  109462,  111932,  114435,
     116971,  119538,  122139,  124772,  127438,  130136,  132868,  135633,
     138432,  141263,  144128,  147027,  149960,  152926,  155926,  158961,
     162029,  165132,  168269,  171441,  174647,  177888,  181164,  184475,
     187821,  191202,  194618,  198069,  201556,  205079,  208637,  212231,
     215861,  219526,  223228,  226966,  230740,  234551,  238398,  242281,
     246201,  250158,  254152,  258183,  262251,  266356,  270498,  274677,
     278894,  283149,  287441,  291771,  296138,  300544,  304987,  309469,
     313989,  318547,  323143,  327778,  332452,  337164,  341914,  346704,
     351533,  356400,  361307,  366253,  371238,  376262,  381326,  386429,
     391572,  396755,  401978,  407240,  412543,  417885,  423268,  428690,
     434154,  439657,  445201,  450786,  456411,  462077,  467784,  473531,
     479320,  485150,  491021,  496933,  502886,  508881,  514918,  520996,
     527115,  533276,  539479,  545724,  552011,  558340,  564712,  571125,
     577580,  584078,  590619,  597202,  603827,  610496,  617207,  623960,
     630757,  637597,  644480,  651406,  658375,  665387,  672443,  679542,
     686685,  693872,  701102,  708376,  715694,  723055,  730461,  737910,
     745404,  752942,  760525,  768151,  775822,  783538,  791298,  799103,
     806952,  814847,  822786,  830770,  838799,  846873,  854993,  863157,
     871367,  879622,  887923,  896269,  904661,  913099,  921582,  930111,
     938686,  947307,  955973,  964686,  973445,  982251,  991102, 1000000,
];

/// Relative luminance, fixed point (x 1_000_000).
const fn luminance(c: Rgb) -> u64 {
    let r = SRGB_LINEAR[c.0 as usize] as u64;
    let g = SRGB_LINEAR[c.1 as usize] as u64;
    let b = SRGB_LINEAR[c.2 as usize] as u64;
    (2126 * r + 7152 * g + 722 * b) / 10_000
}

/// WCAG contrast ratio x 100, so 4.5:1 reads as 450. Integer only, so it const-evaluates.
pub const fn contrast_x100(a: Rgb, b: Rgb) -> u64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    ((hi + 50_000) * 100) / (lo + 50_000)
}

/// Fails the build if `$fg` on `$bg` drops below `$min` x100, naming the pairing.
macro_rules! assert_readable {
    ($fg:expr, $bg:expr, $min:expr, $what:literal) => {
        const _: () = assert!(
            contrast_x100($fg, $bg) >= $min,
            concat!("palette contrast below WCAG minimum: ", $what),
        );
    };
}

// ── the palettes ─────────────────────────────────────────────────────────────

pub struct Palette {
    pub name: &'static str,
    /// Page ground.
    pub paper: Rgb,
    /// Raised surfaces: spec sheets, readouts, code.
    pub sheet: Rgb,
    pub ink: Rgb,
    /// Secondary text: captions, metadata, annotations.
    pub quiet: Rgb,
    /// Links, prompts, the one colour that means "act here".
    pub accent: Rgb,
    /// Hairlines and the notebook grid. Decorative, so not contrast-checked.
    pub rule: Rgb,
    pub grid: Rgb,
}

/// Lab notebook: warm paper, near-black ink, oxide accent.
pub const LIGHT: Palette = Palette {
    name: "light",
    paper: Rgb(0xF4, 0xF1, 0xE9),
    sheet: Rgb(0xFB, 0xF9, 0xF4),
    ink: Rgb(0x17, 0x1A, 0x1F),
    quiet: Rgb(0x58, 0x5C, 0x63),
    accent: Rgb(0x9A, 0x40, 0x12),
    rule: Rgb(0xD9, 0xD2, 0xC2),
    grid: Rgb(0xE9, 0xE4, 0xD8),
};

/// Terminal: graphite ground, paper-white text, ember accent.
pub const DARK: Palette = Palette {
    name: "dark",
    paper: Rgb(0x12, 0x14, 0x17),
    sheet: Rgb(0x1A, 0x1D, 0x21),
    ink: Rgb(0xEC, 0xE7, 0xDC),
    quiet: Rgb(0xA4, 0xA8, 0xAE),
    accent: Rgb(0xF0, 0x8A, 0x4B),
    rule: Rgb(0x2E, 0x33, 0x39),
    grid: Rgb(0x1B, 0x1E, 0x22),
};

// ── the invariants ───────────────────────────────────────────────────────────
//
// Every text colour is held to 4.5:1 against both grounds it can sit on: the page
// and the raised sheet. Captions and annotations are small, so "quiet" gets no
// large-text discount.

assert_readable!(LIGHT.ink, LIGHT.paper, 450, "light ink on paper");
assert_readable!(LIGHT.ink, LIGHT.sheet, 450, "light ink on sheet");
assert_readable!(LIGHT.quiet, LIGHT.paper, 450, "light quiet on paper");
assert_readable!(LIGHT.quiet, LIGHT.sheet, 450, "light quiet on sheet");
assert_readable!(LIGHT.accent, LIGHT.paper, 450, "light accent on paper");
assert_readable!(LIGHT.accent, LIGHT.sheet, 450, "light accent on sheet");

assert_readable!(DARK.ink, DARK.paper, 450, "dark ink on paper");
assert_readable!(DARK.ink, DARK.sheet, 450, "dark ink on sheet");
assert_readable!(DARK.quiet, DARK.paper, 450, "dark quiet on paper");
assert_readable!(DARK.quiet, DARK.sheet, 450, "dark quiet on sheet");
assert_readable!(DARK.accent, DARK.paper, 450, "dark accent on paper");
assert_readable!(DARK.accent, DARK.sheet, 450, "dark accent on sheet");

/// The CSS custom properties for one palette. Contrast figures in the comments are
/// computed from the same values the assertions check, so they cannot go stale.
pub fn css_vars(p: &Palette) -> String {
    let r = |fg: Rgb| contrast_x100(fg, p.paper) as f64 / 100.0;
    format!(
        "  --paper:{paper};--sheet:{sheet};\n\
         \x20 --ink:{ink};       /* {ri:.2}:1 on paper */\n\
         \x20 --quiet:{quiet};     /* {rq:.2}:1 on paper */\n\
         \x20 --accent:{accent};    /* {ra:.2}:1 on paper */\n\
         \x20 --rule:{rule};--grid:{grid};\n\
         \x20 color-scheme:{name};\n",
        paper = p.paper.hex(),
        sheet = p.sheet.hex(),
        ink = p.ink.hex(),
        quiet = p.quiet.hex(),
        accent = p.accent.hex(),
        rule = p.rule.hex(),
        grid = p.grid.hex(),
        name = p.name,
        ri = r(p.ink),
        rq = r(p.quiet),
        ra = r(p.accent),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Anchors the fixed-point maths: black on white is exactly 21:1, anything on
    /// itself exactly 1:1.
    #[test]
    fn known_ratios() {
        let black = Rgb(0, 0, 0);
        let white = Rgb(255, 255, 255);
        assert_eq!(contrast_x100(black, white), 2100);
        assert_eq!(contrast_x100(white, white), 100);
    }

    /// The grid has to stay a whisper. If it ever reads as a pattern behind body text,
    /// the page has turned into graph paper.
    #[test]
    fn grid_stays_faint() {
        for p in [&LIGHT, &DARK] {
            assert!(contrast_x100(p.grid, p.paper) < 115, "{} grid is too strong", p.name);
        }
    }
}
