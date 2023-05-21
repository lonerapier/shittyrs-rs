// Adopted from https://github.com/rust-rse/reed-solomon-erasure/blob/master/build.rs
use std::fs::File;
use std::io::Write;

const PRIME_POLY: usize = 29;

const FIELD_SIZE: usize = 256;
const EXP_TABLE_SIZE: usize = 2 * FIELD_SIZE - 2;

fn gen_log_exp_table(poly: usize) -> ([u8; FIELD_SIZE], [u8; EXP_TABLE_SIZE]) {
    let mut log_table = [0u8; FIELD_SIZE];
    let mut exp_table = [0u8; EXP_TABLE_SIZE];

    let mut x: usize = 1;

    for i in 0..FIELD_SIZE - 1 {
        exp_table[i] = x as u8;
        exp_table[i + FIELD_SIZE - 1] = x as u8;
        log_table[x] = i as u8;

        x <<= 1;

        if x >= FIELD_SIZE {
            x = (x - FIELD_SIZE) ^ poly;
        }
    }

    (log_table, exp_table)
}

macro_rules! write_table {
    ($file:ident, $table:ident, $name:expr, $type:expr) => {
        let len = $table.len();
        let mut table_str = String::from(format!("pub const {}: [{}; {}] = [", $name, $type, len));

        for v in $table.iter() {
            let val_str = format!("{} ,", v);
            table_str.push_str(&val_str);
        }

        table_str.push_str("];\n");

        $file.write_all(table_str.as_bytes()).unwrap();
    };
}

fn generate_tables() {
    let (log_table, exp_table) = gen_log_exp_table(PRIME_POLY);

    let mut f = File::create("src/backend/table.rs").unwrap();

    write_table!(f, log_table, "LOG_TABLE", "u8");
    write_table!(f, exp_table, "EXP_TABLE", "u8");
}

fn main() {
    generate_tables();
}
