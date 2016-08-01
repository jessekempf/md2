use std::fs::File;
use std::io;
use std::io::Read;

const SUBSTITUTION_TABLE: [u8; 256] =
    [41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6, 19, 98, 167, 5, 243, 192,
     199, 115, 140, 152, 147, 43, 217, 188, 76, 130, 202, 30, 155, 87, 60, 253, 212, 224, 22, 103,
     66, 111, 24, 138, 23, 229, 18, 190, 78, 196, 214, 218, 158, 222, 73, 160, 251, 245, 142, 187,
     47, 238, 122, 169, 104, 121, 145, 21, 178, 7, 63, 148, 194, 16, 137, 11, 34, 95, 33, 128,
     127, 93, 154, 90, 144, 50, 39, 53, 62, 204, 231, 191, 247, 151, 3, 255, 25, 48, 179, 72, 165,
     181, 209, 215, 94, 146, 42, 172, 86, 170, 198, 79, 184, 56, 210, 150, 164, 125, 182, 118,
     252, 107, 226, 156, 116, 4, 241, 69, 157, 112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230,
     45, 168, 2, 27, 96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15, 85, 71,
     163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197, 234, 38, 44, 83, 13, 110, 133, 40,
     132, 9, 211, 223, 205, 244, 65, 129, 77, 82, 106, 220, 55, 200, 108, 193, 171, 250, 36, 225,
     123, 8, 12, 189, 177, 74, 120, 136, 149, 139, 227, 99, 232, 109, 233, 203, 213, 254, 59, 0,
     29, 57, 242, 239, 183, 14, 102, 88, 208, 228, 166, 119, 114, 248, 235, 117, 75, 10, 49, 68,
     80, 180, 143, 237, 31, 26, 219, 153, 141, 51, 159, 17, 131, 20];

fn pad_vector(unpadded: &Vec<u8>) -> Vec<u8> {
    let slop_length = unpadded.len() % 16;
    let padding_length = 16 - slop_length;

    let mut padding: Vec<u8> = Vec::with_capacity(padding_length);
    let mut to_pad: Vec<u8> = unpadded.clone();

    let padding_byte: u8 = padding_length as u8;

    for i in 0..padding_length {
        padding.insert(i, padding_byte);
    }

    to_pad.append(&mut padding);
    to_pad
}

fn new_checksum() -> [u8; 16] {
    [0; 16]
}

fn update_checksum(block: &[u8], checksum: &mut [u8; 16]) {
    if block.len() != 16 {
        panic!("update_checksum() must always recieve a 16-byte block");
    }

    let mut l = checksum[15];

    for i in 0..16 {
        let c = checksum[i];
        let b = block[i];

        let new_c = c ^ SUBSTITUTION_TABLE[(b ^ l) as usize];
        checksum[i] = new_c;
        l = new_c;
    }
}

fn run_digest(data: &Vec<u8>) -> [u8; 16] {
    let padded = pad_vector(&data);

    let mut checksum = new_checksum();
    let mut digest = [0; 48];

    for chunk in padded.chunks(16) {
        update_checksum(chunk, &mut checksum);
        update_digest(chunk, &mut digest);
    }

    update_digest(&checksum, &mut digest);

    let mut ret = [0; 16];
    for i in 0..16 {
        ret[i] = digest[i];
    }

    ret
}

fn update_digest(block: &[u8], digest_state: &mut [u8; 48]) {
    if block.len() != 16 {
        panic!("digest() must always recieve a 16-byte block");
    }

    // Copy block into digest state
    for j in 0..16 {
        digest_state[16 + j] = block[j];
        digest_state[32 + j] = digest_state[16 + j] ^ digest_state[j];
    }

    let mut t: u8 = 0;

    for j in 0..18 {
        for k in 0..48 {
            t = digest_state[k] ^ SUBSTITUTION_TABLE[t as usize];
            digest_state[k] = t;
        }

        t = t.wrapping_add(j);
    }
}

fn hex_string(data: &[u8]) -> String {
    let mut rendered = String::with_capacity(2 * data.len());

    for word in data {
        rendered.push_str(&format!("{:x}", word));
    }

    rendered
}

fn md2file(filename: &str) -> io::Result<[u8; 16]> {
    let mut file = try!(File::open(filename));
    let mut data = Vec::new();

    try!(file.read_to_end(&mut data));

    Ok(run_checksum(&data))
}

#[cfg(test)]
mod tests {
    use super::run_checksum;
    use super::hex_string;

    #[test]
    fn it_works() {
        let emptyvec = vec![];
        let emptyMD2 = run_checksum(&emptyvec);

        assert!(hex_string(&emptyMD2) == "8350e5a3e24c153df2275c9f80692773");
    }
}
