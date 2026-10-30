//! Vendor names for MAC addresses, from the IEEE's MA-L, MA-M, MA-S and
//! IAB registries as scripts/make-vendors.py writes them.

use std::collections::HashMap;
use std::sync::OnceLock;

/// An assignment a line: its prefix in hex, 6, 7 or 9 digits for 24, 28
/// or 36 bits, a tab, and its organization.
const LIST: &str = include_str!("vendors.tsv");

/// The organizations by prefix length in digits and prefix.
static NAMES: OnceLock<HashMap<(usize, u64), &'static str>> = OnceLock::new();

/// The company the IEEE assigned the block holding `mac` to, without its
/// legal form.
pub fn name(mac: [u8; 6]) -> Option<String> {
    let names = NAMES.get_or_init(read);
    let [a, b, c, d, e, f] = mac;
    let address = u64::from_be_bytes([0, 0, a, b, c, d, e, f]);
    // The IEEE hands out an MA-L whole or in smaller blocks, never both,
    // so at most one length matches.
    [9, 7, 6]
        .into_iter()
        .find_map(|digits| names.get(&(digits, address >> (48 - 4 * digits))))
        .map(|name| short(name))
}

fn read() -> HashMap<(usize, u64), &'static str> {
    LIST.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let (prefix, name) = line.split_once('\t')?;
            Some(((prefix.len(), u64::from_str_radix(prefix, 16).ok()?), name))
        })
        .collect()
}

/// A company name without its legal form, as "Mark of the Unicorn, Inc."
/// becomes "Mark of the Unicorn".
fn short(name: &str) -> String {
    const FORMS: [&str; 12] = [
        "Inc.",
        "Inc",
        "LLC",
        "Ltd.",
        "Ltd",
        "Limited",
        "GmbH",
        "AG",
        "S.A.",
        "Corp.",
        "Corporation",
        "Co.",
    ];
    let mut name = name.trim();
    loop {
        let before = name;
        for form in FORMS {
            if let Some(rest) = name.strip_suffix(form)
                && rest.ends_with([' ', ','])
            {
                name = rest.trim_end_matches([' ', ',']);
            }
        }
        if name == before {
            return name.to_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_block_size_is_named() {
        assert_eq!(
            name([0x00, 0x01, 0xf2, 0x01, 0x62, 0xe5]).as_deref(),
            Some("Mark of the Unicorn")
        );
        assert_eq!(
            name([0x8c, 0x1f, 0x64, 0x36, 0xc1, 0x00]).as_deref(),
            Some("Scramble Tools")
        );
        let (prefix, organization) = LIST
            .lines()
            .find_map(|line| {
                line.split_once('\t')
                    .filter(|(prefix, _)| prefix.len() == 7)
            })
            .unwrap();
        let address = (u64::from_str_radix(prefix, 16).unwrap() << 20) | 0xabcde;
        let [_, _, mac @ ..] = address.to_be_bytes();
        assert_eq!(name(mac), Some(short(organization)));
    }

    #[test]
    fn the_ieee_names_no_block_it_hands_out() {
        let names = read();
        assert!(names.len() > 50_000);
        assert!(
            !names.iter().any(|(&(digits, _), &organization)| digits == 6
                && organization == "IEEE Registration Authority")
        );
        // 8C-1F-64 is an MA-L the IEEE hands out as MA-S blocks.
        let blocks = 0x8c1f64_u64 << 12;
        let unassigned = (blocks..blocks + 0x1000)
            .find(|prefix| !names.contains_key(&(9, *prefix)))
            .unwrap();
        let [_, _, mac @ ..] = (unassigned << 12).to_be_bytes();
        assert_eq!(name(mac), None);
    }

    #[test]
    fn legal_forms_go() {
        assert_eq!(short("Mark of the Unicorn, Inc."), "Mark of the Unicorn");
        assert_eq!(short("Example Co., Ltd."), "Example");
        assert_eq!(short("Acme"), "Acme");
        assert_eq!(short("Incorporated Things"), "Incorporated Things");
    }
}
