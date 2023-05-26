use base64::{engine::general_purpose, Engine as _};
use shamir::SecretData;

enum Commands {
    SPLIT,
    MERGE,
}
fn main() {
    if std::env::args().len() == 1 {
        println!("split usage: -s <thresholds> <splits> <secret>");
        println!("merge usage: -m <thresholds> <split1> <split2> ... <splitN>");
        return;
    }

    let pattern = std::env::args().nth(1).expect("no command is provided");

    let command = match pattern.as_str() {
        "-s" => Commands::SPLIT,
        "-m" => Commands::MERGE,
        _ => panic!("only -s for split and -m for merge is valid"),
    };

    match command {
        Commands::SPLIT => split(),
        Commands::MERGE => merge(),
    }
}

fn split() {
    let thresh: u8 = std::env::args()
        .nth(2)
        .expect("no threshold # is given")
        .parse()
        .expect("threshold should be number");
    let splits: u8 = std::env::args()
        .nth(3)
        .expect("no splits # is given")
        .parse()
        .expect("splits should be number");
    let secret_data = SecretData::with_secret(
        &std::env::args().nth(4).expect("no secret is given"),
        thresh,
    );
    assert!(
        splits >= thresh,
        "splits should be more than or equal to threshold"
    );
    println!("Key splits:");
    for i in 1..=splits {
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(secret_data.get_share(i).unwrap());
        println!("key {} is  {:?}", i, encoded);
    }
}

fn merge() {
    let thresh: u8 = std::env::args()
        .nth(2)
        .expect("no threshold # is given")
        .parse()
        .expect("threshold should be number");
    let mut splits = vec![];
    for (num, arg) in std::env::args().enumerate() {
        if num < 3 {
            continue;
        }
        let decoded = general_purpose::URL_SAFE_NO_PAD.decode(arg).unwrap();
        splits.push(decoded);
    }

    let recovered = SecretData::recover_secret(thresh, splits).unwrap();
    println!("Recovered: {}", recovered);
}
