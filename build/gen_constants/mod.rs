use std::{env, fs::File, io::Write, path::Path};

#[allow(dead_code)]
mod common;
mod shift_rays;

pub fn generate_constants() -> std::io::Result<()> {
    let common = include_str!("./common.rs").to_string();
    let shift_rays = shift_rays::generate();

    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("gen.rs");
    let mut output = File::create(path).unwrap();

    writeln!(&mut output, "{common}")?;
    writeln!(&mut output, "{shift_rays}")?;
    writeln!(&mut output, "{}", write_rotation_tables())?;
    Ok(())
}

pub fn write_rotation_tables() -> String {
    let (ccw_table, cw_table) = generate_rotation_tables();
    let mut result = String::new();

    result.push_str("pub const CCW_ROTATION_TABLE: [[u64; 256]; 8] = [\n");
    for row in &ccw_table {
        result.push_str("    [");
        for &val in row {
            result.push_str(&format!("0x{val:016x}, "));
        }
        result.push_str("],\n");
    }
    result.push_str("];\n\n");

    result.push_str("pub const CW_ROTATION_TABLE: [[u64; 256]; 8] = [\n");
    for row in &cw_table {
        result.push_str("    [");
        for &val in row {
            result.push_str(&format!("0x{val:016x}, "));
        }
        result.push_str("],\n");
    }
    result.push_str("];\n");

    result
}

pub fn generate_rotation_tables() -> ([[u64; 256]; 8], [[u64; 256]; 8]) {
    let mut ccw_table = [[0u64; 256]; 8];
    let mut cw_table = [[0u64; 256]; 8];

    for row in 0..8 {
        for byte in 0..256u16 {
            let mut ccw_val = 0u64;
            let mut cw_val = 0u64;
            for col in 0..8 {
                if (byte >> col) & 1 != 0 {
                    cw_val |= 1u64 << ((7 - col) * 8 + row);
                    ccw_val |= 1u64 << (col * 8 + (7 - row));
                }
            }
            ccw_table[row][byte as usize] = ccw_val;
            cw_table[row][byte as usize] = cw_val;
        }
    }

    (ccw_table, cw_table)
}
