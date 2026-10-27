//! Bounded structural IWAD validation, before any native engine entry.
use std::collections::HashSet;

pub const MAX_WAD_BYTES: u64 = 128 * 1024 * 1024;

pub fn validate(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 12 || bytes.get(..4) != Some(b"IWAD") {
        return Err("Expected an IWAD file (not a PWAD); select your original shareware DOOM1.WAD".into());
    }
    if bytes.len() as u64 > MAX_WAD_BYTES {
        return Err("IWAD exceeds the 128 MiB MVP limit".into());
    }
    let word = |at: usize| -> Result<usize, String> {
        let raw: [u8; 4] = bytes.get(at..at + 4).and_then(|b| b.try_into().ok()).ok_or("Truncated IWAD directory")?;
        let value = i32::from_le_bytes(raw);
        usize::try_from(value).map_err(|_| "Negative IWAD count or offset".into())
    };
    let count = word(4)?;
    let directory = word(8)?;
    if !(1..=65536).contains(&count) || directory < 12 {
        return Err("Invalid IWAD directory count or offset".into());
    }
    let end = count.checked_mul(16).and_then(|n| directory.checked_add(n)).ok_or("IWAD directory overflow")?;
    if end > bytes.len() {
        return Err("IWAD directory extends beyond the file".into());
    }
    let mut names = HashSet::new();
    for i in 0..count {
        let at = directory + 16 * i;
        let offset = word(at)?;
        let size = word(at + 4)?;
        if offset.checked_add(size).is_none_or(|end| end > bytes.len()) {
            return Err(format!("IWAD lump {i} extends beyond the file"));
        }
        let name = bytes.get(at + 8..at + 16).ok_or("Truncated lump name")?;
        let name = name.split(|c| *c == 0).next().unwrap_or_default();
        names.insert(String::from_utf8_lossy(name).to_ascii_uppercase());
    }
    for name in ["PLAYPAL", "COLORMAP", "PNAMES", "TEXTURE1", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SECTORS", "NODES", "BLOCKMAP"] {
        if !names.contains(name) {
            return Err(format!("IWAD is missing required lump {name}"));
        }
    }
    if !names.contains("E1M1") && !names.contains("MAP01") {
        return Err("IWAD has no supported starting map".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn malformed_headers_and_directories() {
        assert!(validate(b"").is_err());
        assert!(validate(b"PWAD\0\0\0\0\0\0\0\0").is_err());
        for (count, offset) in [(-1i32, 12i32), (i32::MAX, 12), (1, -1), (1, 12)] {
            let mut b = b"IWAD".to_vec();
            b.extend(count.to_le_bytes());
            b.extend(offset.to_le_bytes());
            assert!(validate(&b).is_err());
        }
    }
    #[test]
    fn directory_bounds_and_required_lumps() {
        let names =
            ["PLAYPAL", "COLORMAP", "PNAMES", "TEXTURE1", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SECTORS", "NODES", "BLOCKMAP", "MAP01"];
        let mut b = b"IWAD".to_vec();
        b.extend((names.len() as i32).to_le_bytes());
        b.extend(12i32.to_le_bytes());
        for name in names {
            b.extend(0i32.to_le_bytes());
            b.extend(0i32.to_le_bytes());
            let mut field = [0u8; 8];
            field[..name.len()].copy_from_slice(name.as_bytes());
            b.extend(field);
        }
        assert!(validate(&b).is_ok()); // structural only; not a playable WAD
        b[12..16].copy_from_slice(&i32::MAX.to_le_bytes());
        assert!(validate(&b).is_err());
    }
}
